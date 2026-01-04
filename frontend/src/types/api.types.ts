/**
 * API Type Definitions
 * Generic types for API responses and errors
 */

export interface ApiResponse<T> {
  success: boolean
  data: T
  message?: string
  timestamp: string
}

export interface ApiError {
  success: false
  error: {
    code: string
    message: string
    details?: Record<string, unknown>
  }
  timestamp: string
}

export interface PaginatedResponse<T> {
  success: boolean
  data: T[]
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
    hasNext: boolean
    hasPrev: boolean
  }
  timestamp: string
}

export interface PaginationParams {
  page?: number
  limit?: number
  sortBy?: string
  sortOrder?: 'asc' | 'desc'
}

export interface FilterParams {
  search?: string
  type?: string
  status?: string
  startDate?: string
  endDate?: string
}

export type ApiRequestParams = PaginationParams & FilterParams

/**
 * WebSocket Message Types
 */
export interface WebSocketMessage<T = unknown> {
  type: string
  data: T
  timestamp: string
}

export interface SubscribeMessage {
  type: 'subscribe'
  symbols: string[]
}

export interface UnsubscribeMessage {
  type: 'unsubscribe'
  symbols: string[]
}

export interface PriceUpdateMessage {
  type: 'price_update'
  symbol: string
  price: number
  change24h: number
  volume24h: number
  timestamp: string
}

export interface ErrorMessage {
  type: 'error'
  error: string
  code?: string
}

export type WSMessage =
  | SubscribeMessage
  | UnsubscribeMessage
  | PriceUpdateMessage
  | ErrorMessage

/**
 * HTTP Status Codes
 */
export enum HttpStatus {
  OK = 200,
  CREATED = 201,
  NO_CONTENT = 204,
  BAD_REQUEST = 400,
  UNAUTHORIZED = 401,
  FORBIDDEN = 403,
  NOT_FOUND = 404,
  CONFLICT = 409,
  UNPROCESSABLE_ENTITY = 422,
  TOO_MANY_REQUESTS = 429,
  INTERNAL_SERVER_ERROR = 500,
  SERVICE_UNAVAILABLE = 503,
}

/**
 * API Error Codes
 */
export enum ApiErrorCode {
  // Authentication
  INVALID_CREDENTIALS = 'INVALID_CREDENTIALS',
  TOKEN_EXPIRED = 'TOKEN_EXPIRED',
  TOKEN_INVALID = 'TOKEN_INVALID',
  UNAUTHORIZED = 'UNAUTHORIZED',

  // Validation
  VALIDATION_ERROR = 'VALIDATION_ERROR',
  INVALID_INPUT = 'INVALID_INPUT',
  MISSING_REQUIRED_FIELD = 'MISSING_REQUIRED_FIELD',

  // Resources
  RESOURCE_NOT_FOUND = 'RESOURCE_NOT_FOUND',
  RESOURCE_ALREADY_EXISTS = 'RESOURCE_ALREADY_EXISTS',
  RESOURCE_CONFLICT = 'RESOURCE_CONFLICT',

  // Rate Limiting
  RATE_LIMIT_EXCEEDED = 'RATE_LIMIT_EXCEEDED',

  // Server Errors
  INTERNAL_SERVER_ERROR = 'INTERNAL_SERVER_ERROR',
  SERVICE_UNAVAILABLE = 'SERVICE_UNAVAILABLE',
  DATABASE_ERROR = 'DATABASE_ERROR',

  // External Services
  EXTERNAL_API_ERROR = 'EXTERNAL_API_ERROR',
  MARKET_DATA_UNAVAILABLE = 'MARKET_DATA_UNAVAILABLE',
}
