/**
 * Asset Service
 * Handles asset-related API calls (search, details, history)
 */

import apiClient from './api/client'
import { API_ENDPOINTS } from '@/utils'
import {
  mapAsset,
  mapSearchResults,
  mapChartData,
  type BackendAsset,
  type BackendSearchResponse,
  type BackendPriceHistory,
} from '@/utils/mappers'
import type {
  Asset,
  AssetSearchResult,
  ChartDataPoint,
  Timeframe,
} from '@/types'

/**
 * Search for assets by query string
 */
export async function searchAssets(query: string): Promise<AssetSearchResult[]> {
  if (!query.trim()) {
    return []
  }

  const response = await apiClient.get<BackendSearchResponse>(API_ENDPOINTS.ASSET_SEARCH, {
    params: { q: query },
  })

  return mapSearchResults(response.data)
}

/**
 * Get asset details including current price
 */
export async function getAsset(symbol: string): Promise<Asset> {
  const response = await apiClient.get<BackendAsset>(API_ENDPOINTS.ASSET_DETAILS(symbol))
  return mapAsset(response.data)
}

/**
 * Get asset price history for chart
 */
export async function getAssetHistory(
  symbol: string,
  timeframe: Timeframe = '1D'
): Promise<ChartDataPoint[]> {
  const response = await apiClient.get<BackendPriceHistory>(
    API_ENDPOINTS.ASSET_HISTORY(symbol),
    {
      params: { timeframe },
    }
  )

  return mapChartData(response.data)
}

/**
 * Refresh asset data (invalidate cache)
 */
export async function refreshAsset(symbol: string): Promise<Asset> {
  const response = await apiClient.post<BackendAsset>(API_ENDPOINTS.ASSET_REFRESH(symbol))
  return mapAsset(response.data)
}

/**
 * Get multiple assets by symbols
 */
export async function getAssets(symbols: string[]): Promise<Asset[]> {
  const promises = symbols.map((symbol) => getAsset(symbol))
  const results = await Promise.allSettled(promises)

  return results
    .filter((result): result is PromiseFulfilledResult<Asset> => result.status === 'fulfilled')
    .map((result) => result.value)
}
