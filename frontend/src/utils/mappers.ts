/**
 * API Response Mappers
 * Convert backend snake_case responses to frontend camelCase types
 */

import type {
  User,
  UserPreferences,
  AuthResponse,
} from '@/types/user.types'
import type {
  Asset,
  AssetType,
  AssetSearchResult,
  ChartDataPoint,
} from '@/types/asset.types'
import type {
  Watchlist,
  WatchlistAsset,
} from '@/types/watchlist.types'

// ============================================================================
// Backend Response Types (what the API returns)
// ============================================================================

export interface BackendAuthResponse {
  access_token: string
  refresh_token: string
  token_type: string
  expires_in: number
  user: BackendUserResponse
}

export interface BackendUserResponse {
  id: string
  email: string
  created_at: string
  preferences: BackendUserPreferences
}

export interface BackendUserPreferences {
  theme: string
  default_timeframe: string
  notifications: boolean
}

export interface BackendAsset {
  symbol: string
  name: string
  asset_type: string
  price: number
  change_24h: number
  change_percent_24h: number
  volume_24h: number | null
  market_cap: number | null
  previous_close: number | null
  open: number | null
  high: number | null
  low: number | null
  high_52_week: number | null
  low_52_week: number | null
  last_updated: string
}

export interface BackendSearchResult {
  symbol: string
  name: string
  asset_type: string
  region: string | null
  currency: string | null
  match_score: number | null
}

export interface BackendSearchResponse {
  query: string
  results: BackendSearchResult[]
  total: number
}

export interface BackendHistoricalDataPoint {
  timestamp: string
  open: number
  high: number
  low: number
  close: number
  volume: number
}

export interface BackendPriceHistory {
  symbol: string
  timeframe: string
  data: BackendHistoricalDataPoint[]
  count: number
  fetched_at: string
}

export interface BackendWatchlistAssetResponse {
  symbol: string
  asset_type: string
  added_at: string
}

export interface BackendWatchlistResponse {
  id: string
  name: string
  assets: BackendWatchlistAssetResponse[]
  asset_count: number
  created_at: string
  updated_at: string
}

export interface BackendWatchlistSummary {
  id: string
  name: string
  asset_count: number
  created_at: string
  updated_at: string
}

export interface BackendWatchlistListResponse {
  watchlists: BackendWatchlistSummary[]
  total: number
}

// ============================================================================
// Mapper Functions
// ============================================================================

/**
 * Map backend user preferences to frontend UserPreferences
 */
export function mapUserPreferences(prefs: BackendUserPreferences): UserPreferences {
  return {
    theme: prefs.theme as 'light' | 'dark',
    defaultTimeframe: prefs.default_timeframe,
    defaultAssetType: 'all',
    emailNotifications: prefs.notifications,
    pushNotifications: false,
    priceAlerts: true,
    patternAlerts: true,
    currency: 'USD',
    language: 'en',
  }
}

/**
 * Map backend user response to frontend User
 */
export function mapUserResponse(user: BackendUserResponse): User {
  return {
    id: user.id,
    email: user.email,
    createdAt: user.created_at,
    preferences: mapUserPreferences(user.preferences),
  }
}

/**
 * Map backend auth response to frontend AuthResponse
 */
export function mapAuthResponse(response: BackendAuthResponse): AuthResponse {
  return {
    user: mapUserResponse(response.user),
    token: response.access_token,
    refreshToken: response.refresh_token,
    expiresIn: response.expires_in,
  }
}

/**
 * Map backend asset type string to frontend AssetType
 */
export function mapAssetType(assetType: string): AssetType {
  const type = assetType.toLowerCase()
  if (type === 'stock' || type === 'crypto' || type === 'etf' || type === 'bond') {
    return type as AssetType
  }
  return 'stock' // Default fallback
}

/**
 * Map backend asset to frontend Asset
 */
export function mapAsset(asset: BackendAsset): Asset {
  return {
    id: asset.symbol,
    symbol: asset.symbol,
    name: asset.name,
    type: mapAssetType(asset.asset_type),
    price: asset.price,
    change24h: asset.change_24h,
    changePercent24h: asset.change_percent_24h,
    volume24h: asset.volume_24h ?? 0,
    marketCap: asset.market_cap ?? 0,
    high24h: asset.high ?? undefined,
    low24h: asset.low ?? undefined,
    open24h: asset.open ?? undefined,
    lastUpdated: asset.last_updated,
  }
}

/**
 * Map backend search result to frontend AssetSearchResult
 */
export function mapSearchResult(result: BackendSearchResult): AssetSearchResult {
  return {
    symbol: result.symbol,
    name: result.name,
    type: mapAssetType(result.asset_type),
    exchange: result.region ?? undefined,
  }
}

/**
 * Map backend search response to frontend search results
 */
export function mapSearchResults(response: BackendSearchResponse): AssetSearchResult[] {
  return response.results.map(mapSearchResult)
}

/**
 * Map backend historical data point to frontend ChartDataPoint
 */
export function mapHistoricalDataPoint(point: BackendHistoricalDataPoint): ChartDataPoint {
  // Convert ISO timestamp to Unix timestamp in seconds
  const timestamp = new Date(point.timestamp).getTime() / 1000
  return {
    time: timestamp,
    open: point.open,
    high: point.high,
    low: point.low,
    close: point.close,
    volume: point.volume,
  }
}

/**
 * Map backend price history to frontend chart data
 */
export function mapChartData(history: BackendPriceHistory): ChartDataPoint[] {
  return history.data.map(mapHistoricalDataPoint)
}

/**
 * Map backend watchlist asset to frontend WatchlistAsset
 */
export function mapWatchlistAsset(asset: BackendWatchlistAssetResponse): WatchlistAsset {
  return {
    symbol: asset.symbol,
    name: asset.symbol, // Backend doesn't return name, use symbol as fallback
    type: mapAssetType(asset.asset_type),
    addedAt: asset.added_at,
  }
}

/**
 * Map backend watchlist response to frontend Watchlist
 */
export function mapWatchlist(response: BackendWatchlistResponse): Watchlist {
  return {
    id: response.id,
    userId: '', // Backend doesn't return userId in response
    name: response.name,
    description: undefined,
    isDefault: false, // Backend doesn't have this field
    createdAt: response.created_at,
    updatedAt: response.updated_at,
    assets: response.assets.map(mapWatchlistAsset),
  }
}

/**
 * Map backend watchlist summary to frontend Watchlist (minimal)
 */
export function mapWatchlistSummary(summary: BackendWatchlistSummary): Watchlist {
  return {
    id: summary.id,
    userId: '',
    name: summary.name,
    description: undefined,
    isDefault: false,
    createdAt: summary.created_at,
    updatedAt: summary.updated_at,
    assets: [],
  }
}

/**
 * Map backend watchlist list response to frontend Watchlist array
 */
export function mapWatchlistList(response: BackendWatchlistListResponse): Watchlist[] {
  return response.watchlists.map(mapWatchlistSummary)
}
