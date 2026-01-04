# Phase 1 API Endpoints Reference

This document provides a complete reference for all API endpoints available in the Phase 1 MVP of My-Invest Backend.

## Base URL

```
http://localhost:8080/api/v1
```

## Authentication

All endpoints except `/auth/register`, `/auth/login`, and `/auth/refresh` require authentication.

Include the JWT access token in the Authorization header:

```
Authorization: Bearer <access_token>
```

## Error Response Format

All errors return a consistent JSON format:

```json
{
  "error": "Human-readable error message",
  "status": 400
}
```

### Common Status Codes

| Code | Description |
|------|-------------|
| 200  | OK |
| 201  | Created |
| 400  | Bad Request - Validation error |
| 401  | Unauthorized - Invalid or missing token |
| 403  | Forbidden - Access denied |
| 404  | Not Found - Resource doesn't exist |
| 409  | Conflict - Resource already exists |
| 429  | Too Many Requests - Rate limit exceeded |
| 500  | Internal Server Error |
| 501  | Not Implemented - Feature coming soon |
| 502  | Bad Gateway - External API error |

---

## Authentication Endpoints

### Register

Create a new user account.

**Endpoint:** `POST /api/v1/auth/register`

**Request Body:**

```json
{
  "email": "user@example.com",
  "password": "SecurePassword123",
  "password_confirm": "SecurePassword123"
}
```

**Validation Rules:**
- `email`: Valid email format
- `password`: Minimum 8 characters, must contain at least one letter and one number
- `password_confirm`: Must match `password`

**Response (201 Created):**

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "507f1f77bcf86cd799439011",
    "email": "user@example.com",
    "created_at": "2024-01-15T10:00:00Z",
    "preferences": {
      "theme": "light",
      "default_timeframe": "1D",
      "notifications": true
    }
  }
}
```

**Errors:**
- `400`: Invalid email format or weak password
- `409`: Email already registered

---

### Login

Authenticate an existing user.

**Endpoint:** `POST /api/v1/auth/login`

**Request Body:**

```json
{
  "email": "user@example.com",
  "password": "SecurePassword123"
}
```

**Response (200 OK):**

Same format as register response.

**Errors:**
- `401`: Invalid email or password

---

### Logout

Log out the current user (revoke refresh token).

**Endpoint:** `POST /api/v1/auth/logout`

**Headers:** `Authorization: Bearer <access_token>`

**Request Body (optional):**

```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

If no refresh token is provided, all user's refresh tokens are revoked.

**Response (200 OK):**

```json
{
  "message": "Logged out successfully"
}
```

---

### Refresh Token

Get a new access token using a refresh token.

**Endpoint:** `POST /api/v1/auth/refresh`

**Request Body:**

```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response (200 OK):**

Same format as login response with new tokens.

**Errors:**
- `401`: Invalid or expired refresh token

---

### Get Current User

Get the authenticated user's information.

**Endpoint:** `GET /api/v1/auth/me`

**Headers:** `Authorization: Bearer <access_token>`

**Response (200 OK):**

```json
{
  "id": "507f1f77bcf86cd799439011",
  "email": "user@example.com",
  "created_at": "2024-01-15T10:00:00Z",
  "preferences": {
    "theme": "light",
    "default_timeframe": "1D",
    "notifications": true
  }
}
```

---

### Password Reset (Stub)

Request a password reset. **Not implemented in Phase 1.**

**Endpoint:** `POST /api/v1/auth/reset-password`

**Response (501 Not Implemented):**

```json
{
  "error": "Password reset will be available in a future release. Please contact support for assistance.",
  "status": 501
}
```

---

## Asset Endpoints

### Search Assets

Search for stocks by symbol or company name.

**Endpoint:** `GET /api/v1/assets/search`

**Headers:** `Authorization: Bearer <access_token>`

**Query Parameters:**
- `q` (required): Search query (symbol or company name)

**Example:** `GET /api/v1/assets/search?q=AAPL`

**Response (200 OK):**

```json
{
  "query": "AAPL",
  "results": [
    {
      "symbol": "AAPL",
      "name": "Apple Inc.",
      "asset_type": "stock",
      "region": "United States",
      "currency": "USD",
      "match_score": 1.0
    }
  ],
  "total": 1
}
```

**Notes:**
- Maximum 10 results returned
- Results are cached for 5 minutes
- Phase 1 only returns US stocks

---

### Get Asset Quote

Get current price and details for a stock.

**Endpoint:** `GET /api/v1/assets/:symbol`

**Headers:** `Authorization: Bearer <access_token>`

**Path Parameters:**
- `symbol`: Stock ticker symbol (1-10 alphanumeric characters)

**Example:** `GET /api/v1/assets/AAPL`

**Response (200 OK):**

```json
{
  "symbol": "AAPL",
  "name": "Apple Inc.",
  "asset_type": "stock",
  "price": 152.50,
  "change_24h": 1.50,
  "change_percent_24h": 0.99,
  "volume_24h": 50000000,
  "market_cap": null,
  "previous_close": 151.00,
  "open": 150.00,
  "high": 155.00,
  "low": 149.00,
  "high_52_week": null,
  "low_52_week": null,
  "last_updated": "2024-01-15T16:00:00Z"
}
```

**Errors:**
- `400`: Invalid symbol format
- `404`: Stock symbol not found

---

### Get Asset History

Get historical price data for a stock.

**Endpoint:** `GET /api/v1/assets/:symbol/history`

**Headers:** `Authorization: Bearer <access_token>`

**Path Parameters:**
- `symbol`: Stock ticker symbol

**Query Parameters:**
- `timeframe` (optional): One of `1D`, `1W`, `1M`, `1Y` (default: `1D`)

**Example:** `GET /api/v1/assets/AAPL/history?timeframe=1M`

**Response (200 OK):**

```json
{
  "symbol": "AAPL",
  "timeframe": "1M",
  "data": [
    {
      "timestamp": "2024-01-15T16:00:00Z",
      "open": 150.00,
      "high": 155.00,
      "low": 149.00,
      "close": 152.50,
      "volume": 50000000
    }
  ],
  "count": 30,
  "fetched_at": "2024-01-15T17:00:00Z"
}
```

**Errors:**
- `400`: Invalid symbol or timeframe
- `404`: Stock symbol not found

---

### Refresh Asset Data

Force refresh cached data for a stock.

**Endpoint:** `POST /api/v1/assets/:symbol/refresh`

**Headers:** `Authorization: Bearer <access_token>`

**Response (200 OK):**

Returns fresh quote data in same format as Get Asset Quote.

---

## Watchlist Endpoints

### List Watchlists

Get all watchlists for the authenticated user.

**Endpoint:** `GET /api/v1/watchlists`

**Headers:** `Authorization: Bearer <access_token>`

**Response (200 OK):**

```json
{
  "watchlists": [
    {
      "id": "507f1f77bcf86cd799439011",
      "name": "Tech Stocks",
      "asset_count": 5,
      "created_at": "2024-01-14T10:00:00Z",
      "updated_at": "2024-01-15T11:00:00Z"
    }
  ],
  "total": 1
}
```

---

### Create Watchlist

Create a new watchlist.

**Endpoint:** `POST /api/v1/watchlists`

**Headers:** `Authorization: Bearer <access_token>`

**Request Body:**

```json
{
  "name": "My Portfolio"
}
```

**Validation Rules:**
- `name`: 1-50 characters

**Response (201 Created):**

```json
{
  "id": "507f1f77bcf86cd799439011",
  "name": "My Portfolio",
  "assets": [],
  "asset_count": 0,
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

**Errors:**
- `400`: Invalid name
- `409`: Watchlist with this name already exists
- `429`: Maximum 20 watchlists reached

---

### Get Watchlist

Get a specific watchlist with all assets.

**Endpoint:** `GET /api/v1/watchlists/:id`

**Headers:** `Authorization: Bearer <access_token>`

**Path Parameters:**
- `id`: Watchlist ID (24-character hex string)

**Response (200 OK):**

```json
{
  "id": "507f1f77bcf86cd799439011",
  "name": "Tech Stocks",
  "assets": [
    {
      "symbol": "AAPL",
      "asset_type": "stock",
      "added_at": "2024-01-15T10:00:00Z"
    }
  ],
  "asset_count": 1,
  "created_at": "2024-01-14T10:00:00Z",
  "updated_at": "2024-01-15T11:00:00Z"
}
```

**Errors:**
- `400`: Invalid ID format
- `404`: Watchlist not found

---

### Update Watchlist

Rename a watchlist.

**Endpoint:** `PUT /api/v1/watchlists/:id`

**Headers:** `Authorization: Bearer <access_token>`

**Request Body:**

```json
{
  "name": "Renamed Portfolio"
}
```

**Response (200 OK):**

Returns updated watchlist data.

**Errors:**
- `400`: Invalid name
- `404`: Watchlist not found
- `409`: Watchlist with this name already exists

---

### Delete Watchlist

Delete a watchlist and all its assets.

**Endpoint:** `DELETE /api/v1/watchlists/:id`

**Headers:** `Authorization: Bearer <access_token>`

**Response (200 OK):**

```json
{
  "message": "Watchlist deleted successfully"
}
```

**Errors:**
- `404`: Watchlist not found

---

### Add Asset to Watchlist

Add a stock to a watchlist.

**Endpoint:** `POST /api/v1/watchlists/:id/assets`

**Headers:** `Authorization: Bearer <access_token>`

**Request Body:**

```json
{
  "symbol": "AAPL"
}
```

**Response (201 Created):**

Returns updated watchlist data.

**Errors:**
- `400`: Invalid symbol
- `404`: Watchlist not found or stock symbol not found
- `409`: Symbol already in watchlist
- `429`: Maximum 50 assets per watchlist

---

### Remove Asset from Watchlist

Remove a stock from a watchlist.

**Endpoint:** `DELETE /api/v1/watchlists/:id/assets/:symbol`

**Headers:** `Authorization: Bearer <access_token>`

**Response (200 OK):**

Returns updated watchlist data.

**Errors:**
- `404`: Watchlist not found or symbol not in watchlist

---

## Health Check

### Health Status

Check if the API is healthy.

**Endpoint:** `GET /health`

**Response (200 OK):**

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "service": "my-invest-backend"
}
```

---

## Rate Limiting

- **Limit:** 100 requests per minute per user
- **Headers:** When rate limited, the response includes:
  - `Retry-After`: Seconds until you can retry
- **Status:** `429 Too Many Requests`

---

## Caching

| Data Type | TTL |
|-----------|-----|
| Search results | 5 minutes |
| Stock quotes | 5 minutes |
| Historical data | 1 hour |

To force a refresh, use the `/assets/:symbol/refresh` endpoint.
