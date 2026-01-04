/**
 * Watchlist Type Definitions
 * Types for managing custom asset watchlists
 */

import { AssetType } from './asset.types'

export interface Watchlist {
  id: string
  userId: string
  name: string
  description?: string
  isDefault: boolean
  createdAt: string
  updatedAt: string
  assets: WatchlistAsset[]
}

export interface WatchlistAsset {
  symbol: string
  name: string
  type: AssetType
  addedAt: string
  notes?: string
  targetPrice?: number
  stopLoss?: number
}

export interface CreateWatchlistDto {
  name: string
  description?: string
  isDefault?: boolean
}

export interface UpdateWatchlistDto {
  name?: string
  description?: string
  isDefault?: boolean
}

export interface AddAssetToWatchlistDto {
  symbol: string
  name: string
  type: AssetType
  notes?: string
  targetPrice?: number
  stopLoss?: number
}

export interface RemoveAssetFromWatchlistDto {
  symbol: string
}

export interface WatchlistSummary {
  id: string
  name: string
  assetCount: number
  isDefault: boolean
  createdAt: string
  updatedAt: string
}
