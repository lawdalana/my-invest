/**
 * Watchlist Service
 * Handles watchlist CRUD operations and asset management
 */

import apiClient from './api/client'
import { API_ENDPOINTS } from '@/utils'
import {
  mapWatchlist,
  mapWatchlistList,
  type BackendWatchlistResponse,
  type BackendWatchlistListResponse,
} from '@/utils/mappers'
import type { Watchlist } from '@/types'

/**
 * Get all watchlists for current user
 */
export async function getAllWatchlists(): Promise<Watchlist[]> {
  const response = await apiClient.get<BackendWatchlistListResponse>(API_ENDPOINTS.WATCHLISTS)
  return mapWatchlistList(response.data)
}

/**
 * Get a specific watchlist by ID
 */
export async function getWatchlist(id: string): Promise<Watchlist> {
  const response = await apiClient.get<BackendWatchlistResponse>(
    API_ENDPOINTS.WATCHLIST_DETAILS(id)
  )
  return mapWatchlist(response.data)
}

/**
 * Create a new watchlist
 */
export async function createWatchlist(name: string): Promise<Watchlist> {
  const response = await apiClient.post<BackendWatchlistResponse>(API_ENDPOINTS.WATCHLISTS, {
    name,
  })
  return mapWatchlist(response.data)
}

/**
 * Update an existing watchlist
 */
export async function updateWatchlist(id: string, name: string): Promise<Watchlist> {
  const response = await apiClient.put<BackendWatchlistResponse>(
    API_ENDPOINTS.WATCHLIST_DETAILS(id),
    { name }
  )
  return mapWatchlist(response.data)
}

/**
 * Delete a watchlist
 */
export async function deleteWatchlist(id: string): Promise<void> {
  await apiClient.delete(API_ENDPOINTS.WATCHLIST_DETAILS(id))
}

/**
 * Add an asset to a watchlist
 */
export async function addAssetToWatchlist(
  watchlistId: string,
  symbol: string
): Promise<Watchlist> {
  const response = await apiClient.post<BackendWatchlistResponse>(
    API_ENDPOINTS.ADD_TO_WATCHLIST(watchlistId),
    { symbol }
  )
  return mapWatchlist(response.data)
}

/**
 * Remove an asset from a watchlist
 */
export async function removeAssetFromWatchlist(
  watchlistId: string,
  symbol: string
): Promise<Watchlist> {
  const response = await apiClient.delete<BackendWatchlistResponse>(
    API_ENDPOINTS.REMOVE_FROM_WATCHLIST(watchlistId, symbol)
  )
  return mapWatchlist(response.data)
}
