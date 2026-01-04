/**
 * Dashboard Page
 * Main dashboard with asset grid and filters
 */

import { useState, useEffect } from 'react'
import { Navigation, AssetCard, SkeletonAssetCard } from '@/components'
import { Asset, AssetType } from '@/types'
import { ASSET_TYPE_LABELS, ASSET_TYPES } from '@/utils'

// Mock data for development
const MOCK_ASSETS: Asset[] = [
  {
    id: '1',
    symbol: 'AAPL',
    name: 'Apple Inc.',
    type: 'stock',
    price: 178.25,
    change24h: 2.34,
    changePercent24h: 2.34,
    volume24h: 52840000,
    marketCap: 2890000000000,
    lastUpdated: new Date().toISOString(),
  },
  {
    id: '2',
    symbol: 'BTC',
    name: 'Bitcoin',
    type: 'crypto',
    price: 43250.50,
    change24h: -1.25,
    changePercent24h: -1.25,
    volume24h: 28500000000,
    marketCap: 845000000000,
    lastUpdated: new Date().toISOString(),
  },
  {
    id: '3',
    symbol: 'SPY',
    name: 'SPDR S&P 500 ETF',
    type: 'etf',
    price: 468.32,
    change24h: 0.87,
    changePercent24h: 0.87,
    volume24h: 75200000,
    marketCap: 425000000000,
    lastUpdated: new Date().toISOString(),
  },
]

export function DashboardPage() {
  const [assets, setAssets] = useState<Asset[]>([])
  const [filteredAssets, setFilteredAssets] = useState<Asset[]>([])
  const [selectedType, setSelectedType] = useState<AssetType | 'all'>('all')
  const [isLoading, setIsLoading] = useState(true)

  // Load assets
  useEffect(() => {
    const loadAssets = async () => {
      setIsLoading(true)

      // Simulate API call
      await new Promise((resolve) => setTimeout(resolve, 1000))

      setAssets(MOCK_ASSETS)
      setIsLoading(false)
    }

    loadAssets()
  }, [])

  // Filter assets by type
  useEffect(() => {
    if (selectedType === 'all') {
      setFilteredAssets(assets)
    } else {
      setFilteredAssets(assets.filter((asset) => asset.type === selectedType))
    }
  }, [assets, selectedType])

  return (
    <div className="min-h-screen bg-background-primary">
      <Navigation />

      <main className="pt-20 pb-8">
        <div className="container-custom">
          {/* Header */}
          <div className="mb-6">
            <h1 className="text-3xl font-bold text-text-primary mb-2">
              Dashboard
            </h1>
            <p className="text-text-secondary">
              Track your favorite assets in real-time
            </p>
          </div>

          {/* Filters */}
          <div className="flex gap-2 mb-6 overflow-x-auto no-scrollbar">
            <button
              onClick={() => setSelectedType('all')}
              className={`px-4 py-2 rounded border transition-colors ${
                selectedType === 'all'
                  ? 'bg-accent text-white border-accent'
                  : 'bg-background-secondary text-text-secondary border-border-primary hover:border-border-hover'
              }`}
            >
              All
            </button>
            {ASSET_TYPES.map((type) => (
              <button
                key={type}
                onClick={() => setSelectedType(type)}
                className={`px-4 py-2 rounded border transition-colors whitespace-nowrap ${
                  selectedType === type
                    ? 'bg-accent text-white border-accent'
                    : 'bg-background-secondary text-text-secondary border-border-primary hover:border-border-hover'
                }`}
              >
                {ASSET_TYPE_LABELS[type]}
              </button>
            ))}
          </div>

          {/* Asset Grid */}
          {isLoading ? (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {[1, 2, 3, 4, 5, 6].map((i) => (
                <SkeletonAssetCard key={i} />
              ))}
            </div>
          ) : filteredAssets.length > 0 ? (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {filteredAssets.map((asset) => (
                <AssetCard key={asset.id} asset={asset} />
              ))}
            </div>
          ) : (
            <div className="text-center py-12">
              <p className="text-text-secondary text-lg">
                No assets found for this filter
              </p>
            </div>
          )}
        </div>
      </main>
    </div>
  )
}
