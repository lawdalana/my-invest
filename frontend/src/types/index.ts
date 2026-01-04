/**
 * Type Definitions Index
 * Central export point for all application types
 */

// Asset Types
export type {
  AssetType,
  Timeframe,
  Asset,
  ChartDataPoint,
  AssetDetails,
  AssetSearchResult,
  AssetPriceUpdate,
} from './asset.types'

// User Types
export type {
  User,
  UserPreferences,
  LoginCredentials,
  RegisterData,
  AuthResponse,
  AuthState,
  PasswordResetRequest,
  PasswordReset,
  ChangePassword,
  UpdateProfile,
  UpdatePreferences,
} from './user.types'

// Watchlist Types
export type {
  Watchlist,
  WatchlistAsset,
  CreateWatchlistDto,
  UpdateWatchlistDto,
  AddAssetToWatchlistDto,
  RemoveAssetFromWatchlistDto,
  WatchlistSummary,
} from './watchlist.types'

// Alert Types
export type {
  AlertType,
  PriceCondition,
  PatternType,
  AlertStatus,
  Alert,
  CreatePriceAlertDto,
  CreatePatternAlertDto,
  CreateAlertDto,
  UpdateAlertDto,
  AlertSummary,
  AlertNotification,
} from './alert.types'

// API Types
export type {
  ApiResponse,
  ApiError,
  PaginatedResponse,
  PaginationParams,
  FilterParams,
  ApiRequestParams,
  WebSocketMessage,
  SubscribeMessage,
  UnsubscribeMessage,
  PriceUpdateMessage,
  ErrorMessage,
  WSMessage,
} from './api.types'

export { HttpStatus, ApiErrorCode } from './api.types'
