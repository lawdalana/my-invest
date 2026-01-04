/**
 * Asset Type Definitions
 * Types for stocks, cryptocurrencies, ETFs, and bonds
 */

export type AssetType = 'stock' | 'crypto' | 'etf' | 'bond'

export type Timeframe = '1D' | '1W' | '1M' | '3M' | '6M' | '1Y' | '5Y' | 'MAX'

export interface Asset {
  id: string
  symbol: string
  name: string
  type: AssetType
  price: number
  change24h: number // Percentage change in last 24 hours
  changePercent24h: number // Same as change24h, for consistency
  volume24h: number
  marketCap: number
  high24h?: number
  low24h?: number
  open24h?: number
  lastUpdated: string // ISO 8601 timestamp
}

export interface ChartDataPoint {
  time: number // Unix timestamp in seconds
  open: number
  high: number
  low: number
  close: number
  volume?: number
}

export interface AssetDetails extends Asset {
  description?: string
  sector?: string // For stocks
  industry?: string // For stocks
  website?: string
  headquarters?: string // For stocks
  ceo?: string // For stocks
  founded?: string // For stocks
  employees?: number // For stocks
  marketCapRank?: number // For crypto
  circulatingSupply?: number // For crypto
  totalSupply?: number // For crypto
  maxSupply?: number // For crypto
  allTimeHigh?: number
  allTimeHighDate?: string
  allTimeLow?: number
  allTimeLowDate?: string
  priceChange7d?: number
  priceChange30d?: number
  priceChange1y?: number
}

export interface AssetSearchResult {
  symbol: string
  name: string
  type: AssetType
  exchange?: string
}

export interface AssetPriceUpdate {
  symbol: string
  price: number
  change24h: number
  timestamp: string
}
