/**
 * Dashboard Page
 * Main dashboard with asset grid and filters
 */

import { useState, useEffect, useCallback } from 'react'
import { Navigation, AssetCard, SkeletonAssetCard } from '@/components'
import { Asset, AssetType, AssetSearchResult } from '@/types'
import { ASSET_TYPE_LABELS, ASSET_TYPES } from '@/utils'
import { useWatchlists } from '@/hooks'
import * as assetService from '@/services/asset.service'
import { getErrorMessage } from '@/services/api/client'
import { useNotification } from '@/contexts/NotificationContext'

export function DashboardPage() {
  const [assets, setAssets] = useState<Asset[]>([])
  const [filteredAssets, setFilteredAssets] = useState<Asset[]>([])
  const [selectedType, setSelectedType] = useState<AssetType | 'all'>('all')
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const { watchlists, isLoading: watchlistsLoading } = useWatchlists()
  const { showToast } = useNotification()

  // Load assets from watchlists
  useEffect(() => {
    const loadAssets = async () => {
      // Wait for watchlists to load
      if (watchlistsLoading) return

      setIsLoading(true)
      setError(null)

      try {
        // Collect all unique symbols from all watchlists
        const allSymbols = new Set<string>()
        watchlists.forEach((watchlist) => {
          watchlist.assets.forEach((asset) => {
            allSymbols.add(asset.symbol)
          })
        })

        if (allSymbols.size === 0) {
          // No assets in watchlists
          setAssets([])
          setIsLoading(false)
          return
        }

        // Fetch details for all symbols
        const assetPromises = Array.from(allSymbols).map((symbol) =>
          assetService.getAsset(symbol).catch((err) => {
            console.warn(`Failed to fetch asset ${symbol}:`, err)
            return null
          })
        )

        const results = await Promise.all(assetPromises)
        const validAssets = results.filter((asset): asset is Asset => asset !== null)

        setAssets(validAssets)
      } catch (err) {
        const errorMessage = getErrorMessage(err)
        setError(errorMessage)
        showToast('Failed to load assets', 'error')
      } finally {
        setIsLoading(false)
      }
    }

    loadAssets()
  }, [watchlists, watchlistsLoading, showToast])

  // Filter assets by type
  useEffect(() => {
    if (selectedType === 'all') {
      setFilteredAssets(assets)
    } else {
      setFilteredAssets(assets.filter((asset) => asset.type === selectedType))
    }
  }, [assets, selectedType])

  // Handle search
  const handleSearch = useCallback(
    async (query: string): Promise<AssetSearchResult[]> => {
      try {
        return await assetService.searchAssets(query)
      } catch (err) {
        console.error('Search failed:', err)
        showToast('Search failed', 'error')
        return []
      }
    },
    [showToast]
  )

  const showLoading = isLoading || watchlistsLoading

  return (
    <div className="min-h-screen bg-background-primary">
      <Navigation onSearch={handleSearch} />

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

          {/* Error message */}
          {error && (
            <div className="mb-6 p-4 bg-danger/10 border border-danger rounded-lg">
              <p className="text-danger">{error}</p>
            </div>
          )}

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
          {showLoading ? (
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
          ) : assets.length === 0 ? (
            <div className="text-center py-12">
              <p className="text-text-secondary text-lg mb-4">
                No assets in your watchlists yet
              </p>
              <p className="text-text-tertiary">
                Search for assets using the search bar above and add them to a watchlist
              </p>
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
