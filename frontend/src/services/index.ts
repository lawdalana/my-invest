/**
 * Services Index
 * Re-exports all API services for convenient importing
 */

// API Client
export { default as apiClient, getErrorMessage, apiRequest } from './api/client'

// Authentication
export * as authService from './auth.service'

// Assets
export * as assetService from './asset.service'

// Watchlists
export * as watchlistService from './watchlist.service'
