//! WebSocket HTTP handler
//!
//! This module handles WebSocket upgrade requests and manages the WebSocket connection lifecycle.
//! Authentication is done via a `token` query parameter.

use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use tokio::sync::mpsc;
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

use crate::models::websocket::{WsClientMessage, WsServerMessage};
use crate::services::ws_service::WsService;
use crate::utils::error::AppError;
use crate::utils::jwt::JwtManager;

/// Query parameters for WebSocket connection
#[derive(Debug, Deserialize)]
pub struct WsConnectQuery {
    /// JWT token for authentication (optional - allows unauthenticated connections too)
    pub token: Option<String>,
}

/// State for the WebSocket handler
#[derive(Clone)]
pub struct WsHandlerState {
    pub ws_service: Arc<WsService>,
    pub jwt_manager: Arc<JwtManager>,
}

/// Handle WebSocket upgrade request
///
/// GET /api/v1/ws?token=jwt_token
///
/// This handler:
/// 1. Optionally validates the JWT token (if provided)
/// 2. Upgrades the HTTP connection to WebSocket
/// 3. Manages the WebSocket connection lifecycle
#[instrument(skip(state, ws))]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<WsHandlerState>,
    Query(query): Query<WsConnectQuery>,
) -> impl IntoResponse {
    // Optionally validate the JWT token
    let user_info = if let Some(token) = &query.token {
        match state.jwt_manager.validate_access_token(token) {
            Ok(claims) => {
                info!(user_id = %claims.sub, "WebSocket connection authenticated");
                Some((claims.sub, claims.email))
            }
            Err(e) => {
                warn!(error = %e, "Invalid WebSocket authentication token");
                // Allow connection but mark as unauthenticated
                None
            }
        }
    } else {
        debug!("WebSocket connection without authentication");
        None
    };

    // Upgrade to WebSocket
    ws.on_upgrade(move |socket| handle_socket(socket, state, user_info))
}

/// Handle the WebSocket connection after upgrade
async fn handle_socket(
    socket: WebSocket,
    state: WsHandlerState,
    user_info: Option<(String, String)>,
) {
    // Generate a unique connection ID
    let connection_id = Uuid::new_v4().to_string();

    let user_id = user_info.as_ref().map(|(id, _)| id.clone());

    info!(
        connection_id = %connection_id,
        user_id = ?user_id,
        "WebSocket connection established"
    );

    // Create channel for sending messages to this connection
    let (msg_tx, mut msg_rx) = mpsc::channel::<WsServerMessage>(100);

    // Register connection with the service
    state.ws_service.connect(connection_id.clone(), msg_tx).await;

    // Split the WebSocket into sender and receiver
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Spawn task to forward messages from the service to the WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = msg_rx.recv().await {
            match serde_json::to_string(&msg) {
                Ok(json) => {
                    if let Err(e) = ws_sender.send(Message::Text(json)).await {
                        warn!(error = %e, "Failed to send WebSocket message");
                        break;
                    }
                }
                Err(e) => {
                    error!(error = %e, "Failed to serialize WebSocket message");
                }
            }
        }
    });

    // Handle incoming messages from the client
    let ws_service = state.ws_service.clone();
    let conn_id = connection_id.clone();

    let recv_task = tokio::spawn(async move {
        while let Some(result) = ws_receiver.next().await {
            match result {
                Ok(msg) => {
                    if !handle_client_message(msg, &ws_service, &conn_id).await {
                        break;
                    }
                }
                Err(e) => {
                    warn!(
                        connection_id = %conn_id,
                        error = %e,
                        "WebSocket receive error"
                    );
                    break;
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {
            debug!(connection_id = %connection_id, "Send task completed");
        }
        _ = recv_task => {
            debug!(connection_id = %connection_id, "Receive task completed");
        }
    }

    // Cleanup: disconnect from service
    state.ws_service.disconnect(&connection_id).await;

    info!(
        connection_id = %connection_id,
        user_id = ?user_id,
        "WebSocket connection closed"
    );
}

/// Handle a single message from the client
///
/// Returns `true` to continue processing, `false` to close the connection.
async fn handle_client_message(msg: Message, ws_service: &WsService, connection_id: &str) -> bool {
    match msg {
        Message::Text(text) => {
            // Parse the message
            match serde_json::from_str::<WsClientMessage>(&text) {
                Ok(client_msg) => {
                    process_client_message(client_msg, ws_service, connection_id).await;
                }
                Err(e) => {
                    warn!(
                        connection_id = %connection_id,
                        error = %e,
                        text = %text,
                        "Failed to parse client message"
                    );

                    // Send error back to client
                    // Note: We need to send via the service since we don't have direct access
                    // to the sender here. In practice, the error will be logged.
                }
            }
            true
        }
        Message::Binary(_) => {
            debug!(connection_id = %connection_id, "Received binary message (ignored)");
            true
        }
        Message::Ping(_data) => {
            debug!(connection_id = %connection_id, "Received ping");
            // The WebSocket library handles pong automatically
            true
        }
        Message::Pong(_) => {
            debug!(connection_id = %connection_id, "Received pong");
            true
        }
        Message::Close(_) => {
            info!(connection_id = %connection_id, "Received close frame");
            false
        }
    }
}

/// Process a parsed client message
async fn process_client_message(
    msg: WsClientMessage,
    ws_service: &WsService,
    connection_id: &str,
) {
    match msg {
        WsClientMessage::Subscribe { symbols } => {
            debug!(
                connection_id = %connection_id,
                symbols = ?symbols,
                "Processing subscribe request"
            );
            ws_service.subscribe(connection_id, symbols).await;
        }
        WsClientMessage::Unsubscribe { symbols } => {
            debug!(
                connection_id = %connection_id,
                symbols = ?symbols,
                "Processing unsubscribe request"
            );
            ws_service.unsubscribe(connection_id, symbols).await;
        }
        WsClientMessage::Ping => {
            debug!(connection_id = %connection_id, "Processing ping request");
            // Note: The pong is sent via the service channel
            // This is an application-level ping, not a WebSocket protocol ping
        }
    }
}

/// Alternative handler that requires authentication
///
/// Use this handler if you want to require authentication for WebSocket connections.
#[allow(dead_code)]
#[instrument(skip(state, ws))]
pub async fn ws_handler_authenticated(
    ws: WebSocketUpgrade,
    State(state): State<WsHandlerState>,
    Query(query): Query<WsConnectQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Require the token parameter
    let token = query.token.ok_or_else(|| {
        AppError::Unauthorized("Authentication token required for WebSocket connection".to_string())
    })?;

    // Validate the JWT token
    let claims = state.jwt_manager.validate_access_token(&token)?;

    info!(
        user_id = %claims.sub,
        email = %claims.email,
        "Authenticated WebSocket connection"
    );

    let user_info = Some((claims.sub, claims.email));

    // Upgrade to WebSocket
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state, user_info)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_connect_query_deserialize() {
        let json = r#"{"token": "abc123"}"#;
        let query: WsConnectQuery = serde_json::from_str(json).unwrap();
        assert_eq!(query.token, Some("abc123".to_string()));
    }

    #[test]
    fn test_ws_connect_query_no_token() {
        let json = r#"{}"#;
        let query: WsConnectQuery = serde_json::from_str(json).unwrap();
        assert_eq!(query.token, None);
    }
}
