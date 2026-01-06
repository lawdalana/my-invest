/**
 * Application Constants
 * Central location for all application constants
 */

import { AssetType, Timeframe } from '@/types'

/**
 * Chart Timeframes
 */
export const TIMEFRAMES: Timeframe[] = [
  '1D',
  '1W',
  '1M',
  '3M',
  '6M',
  '1Y',
  '5Y',
  'MAX',
]

export const TIMEFRAME_LABELS: Record<Timeframe, string> = {
  '1D': '1 Day',
  '1W': '1 Week',
  '1M': '1 Month',
  '3M': '3 Months',
  '6M': '6 Months',
  '1Y': '1 Year',
  '5Y': '5 Years',
  MAX: 'Max',
}

/**
 * Asset Types
 */
export const ASSET_TYPES: AssetType[] = ['stock', 'crypto', 'etf', 'bond']

export const ASSET_TYPE_LABELS: Record<AssetType, string> = {
  stock: 'Stocks',
  crypto: 'Crypto',
  etf: 'ETFs',
  bond: 'Bonds',
}

export const ASSET_TYPE_ICONS: Record<AssetType, string> = {
  stock: 'TrendingUp',
  crypto: 'Bitcoin',
  etf: 'Briefcase',
  bond: 'FileText',
}

/**
 * Refresh Intervals (in milliseconds)
 */
export const REFRESH_INTERVALS = {
  PRICE_UPDATE: 5000, // 5 seconds
  WATCHLIST: 30000, // 30 seconds
  ALERTS_CHECK: 60000, // 1 minute
  CHART_DATA: 60000, // 1 minute
} as const

/**
 * Local Storage Keys
 */
export const LOCAL_STORAGE_KEYS = {
  AUTH_TOKEN: 'my-invest-auth-token',
  REFRESH_TOKEN: 'my-invest-refresh-token',
  USER_DATA: 'my-invest-user-data',
  THEME: 'my-invest-theme',
  PREFERENCES: 'my-invest-preferences',
  MOCK_USERS: 'my-invest-mock-users',
  LAST_VIEWED_ASSETS: 'my-invest-last-viewed-assets',
} as const

/**
 * API Endpoints
 * Note: All endpoints are prefixed with /v1 by the backend
 */
export const API_ENDPOINTS = {
  // Authentication
  LOGIN: '/v1/auth/login',
  REGISTER: '/v1/auth/register',
  LOGOUT: '/v1/auth/logout',
  REFRESH_TOKEN: '/v1/auth/refresh',
  CURRENT_USER: '/v1/auth/me',
  RESET_PASSWORD: '/v1/auth/reset-password',
  RESET_PASSWORD_CONFIRM: '/v1/auth/reset-password/confirm',

  // Assets
  ASSETS: '/v1/assets',
  ASSET_DETAILS: (symbol: string) => `/v1/assets/${symbol}`,
  ASSET_SEARCH: '/v1/assets/search',
  ASSET_HISTORY: (symbol: string) => `/v1/assets/${symbol}/history`,
  ASSET_REFRESH: (symbol: string) => `/v1/assets/${symbol}/refresh`,

  // Watchlists
  WATCHLISTS: '/v1/watchlists',
  WATCHLIST_DETAILS: (id: string) => `/v1/watchlists/${id}`,
  ADD_TO_WATCHLIST: (id: string) => `/v1/watchlists/${id}/assets`,
  REMOVE_FROM_WATCHLIST: (id: string, symbol: string) =>
    `/v1/watchlists/${id}/assets/${symbol}`,

  // Alerts (future implementation)
  ALERTS: '/v1/alerts',
  ALERT_DETAILS: (id: string) => `/v1/alerts/${id}`,
  UPDATE_ALERT: (id: string) => `/v1/alerts/${id}`,
  DELETE_ALERT: (id: string) => `/v1/alerts/${id}`,
  ALERT_SUMMARY: '/v1/alerts/summary',
} as const

/**
 * WebSocket Events
 */
export const WS_EVENTS = {
  CONNECT: 'connect',
  DISCONNECT: 'disconnect',
  ERROR: 'error',
  SUBSCRIBE: 'subscribe',
  UNSUBSCRIBE: 'unsubscribe',
  PRICE_UPDATE: 'price_update',
  ALERT_TRIGGERED: 'alert_triggered',
} as const

/**
 * Price Alert Conditions
 */
export const PRICE_CONDITIONS = [
  { value: 'above', label: 'Above' },
  { value: 'below', label: 'Below' },
  { value: 'crosses_above', label: 'Crosses Above' },
  { value: 'crosses_below', label: 'Crosses Below' },
] as const

/**
 * Pattern Types
 */
export const PATTERN_TYPES = [
  { value: 'double_top', label: 'Double Top' },
  { value: 'double_bottom', label: 'Double Bottom' },
  { value: 'head_shoulders', label: 'Head & Shoulders' },
  { value: 'inverse_head_shoulders', label: 'Inverse Head & Shoulders' },
  { value: 'triangle', label: 'Triangle' },
  { value: 'wedge', label: 'Wedge' },
  { value: 'flag', label: 'Flag' },
  { value: 'pennant', label: 'Pennant' },
] as const

/**
 * Pagination Defaults
 */
export const PAGINATION = {
  DEFAULT_PAGE: 1,
  DEFAULT_LIMIT: 20,
  MAX_LIMIT: 100,
} as const

/**
 * Debounce/Throttle Delays (in milliseconds)
 */
export const DELAYS = {
  SEARCH_DEBOUNCE: 300,
  RESIZE_DEBOUNCE: 150,
  SCROLL_THROTTLE: 100,
  PRICE_UPDATE_THROTTLE: 1000,
} as const

/**
 * Toast Auto-dismiss Duration (in milliseconds)
 */
export const TOAST_DURATION = 5000

/**
 * Chart Colors
 */
export const CHART_COLORS = {
  UP: '#26A69A', // Success/Green
  DOWN: '#EF5350', // Danger/Red
  GRID: '#2A2E39',
  TEXT: '#787B86',
  CROSSHAIR: '#9598A1',
  BACKGROUND: '#131722',
} as const

/**
 * Theme Colors
 */
export const THEME_COLORS = {
  DARK: {
    background: {
      primary: '#131722',
      secondary: '#1E222D',
      tertiary: '#2A2E39',
    },
    text: {
      primary: '#D1D4DC',
      secondary: '#B2B5BE',
      tertiary: '#787B86',
    },
  },
  LIGHT: {
    background: {
      primary: '#FFFFFF',
      secondary: '#F7F9FA',
      tertiary: '#EBEBEB',
    },
    text: {
      primary: '#131722',
      secondary: '#434651',
      tertiary: '#787B86',
    },
  },
} as const

/**
 * Password Requirements
 */
export const PASSWORD_REQUIREMENTS = {
  MIN_LENGTH: 8,
  REQUIRE_UPPERCASE: true,
  REQUIRE_LOWERCASE: true,
  REQUIRE_NUMBER: true,
  REQUIRE_SPECIAL_CHAR: true,
} as const

/**
 * Maximum Items
 */
export const MAX_ITEMS = {
  WATCHLIST_ASSETS: 50,
  WATCHLISTS: 10,
  ACTIVE_ALERTS: 50,
  SEARCH_RESULTS: 10,
  RECENT_SEARCHES: 5,
} as const
