/**
 * Asset Detail Page
 * Detailed asset view with chart and stats
 */

import React, { useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ArrowLeft, Star, Bell } from 'lucide-react'
import { Navigation, ChartContainer, Button, Badge, SkeletonChart } from '@/components'
import { Watchlist } from '@/types'
import { formatCurrency, formatLargeNumber, formatDateTime, ASSET_TYPE_LABELS } from '@/utils'
import { useNotification } from '@/contexts'
import { useAsset, useWatchlists } from '@/hooks'

export function AssetDetailPage() {
  const { symbol } = useParams<{ symbol: string }>()
  const navigate = useNavigate()
  const { showToast } = useNotification()

  const { asset, chartData, isLoading, error, timeframe, setTimeframe } = useAsset(symbol)
  const { watchlists, addAsset } = useWatchlists()

  const [showWatchlistModal, setShowWatchlistModal] = useState(false)
  const [selectedWatchlist, setSelectedWatchlist] = useState<string>('')

  const handleTimeframeChange = (newTimeframe: typeof timeframe) => {
    setTimeframe(newTimeframe)
  }

  const handleAddToWatchlist = () => {
    if (watchlists.length === 0) {
      showToast('Please create a watchlist first', 'info')
      return
    }
    setShowWatchlistModal(true)
  }

  const handleConfirmAddToWatchlist = async () => {
    if (!selectedWatchlist || !symbol) return

    const result = await addAsset(selectedWatchlist, symbol)
    if (result) {
      showToast('Added to watchlist', 'success')
    } else {
      showToast('Failed to add to watchlist', 'error')
    }
    setShowWatchlistModal(false)
    setSelectedWatchlist('')
  }

  const handleCreateAlert = () => {
    showToast('Alert creation coming soon', 'info')
  }

  if (error) {
    return (
      <div className="min-h-screen bg-background-primary">
        <Navigation />
        <main className="pt-20 pb-8">
          <div className="container-custom">
            <div className="text-center py-12">
              <p className="text-danger text-lg mb-4">{error}</p>
              <Button onClick={() => navigate('/')} variant="primary">
                Go to Dashboard
              </Button>
            </div>
          </div>
        </main>
      </div>
    )
  }

  if (!asset && !isLoading) {
    return (
      <div className="min-h-screen bg-background-primary">
        <Navigation />
        <main className="pt-20 pb-8">
          <div className="container-custom">
            <div className="text-center py-12">
              <p className="text-text-secondary text-lg">Asset not found</p>
              <Button onClick={() => navigate('/')} variant="primary" className="mt-4">
                Go to Dashboard
              </Button>
            </div>
          </div>
        </main>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-background-primary">
      <Navigation />

      <main className="pt-20 pb-8">
        <div className="container-custom">
          {/* Back Button */}
          <Button
            variant="ghost"
            onClick={() => navigate('/')}
            className="mb-4"
          >
            <ArrowLeft className="w-4 h-4" />
            Back to Dashboard
          </Button>

          {isLoading || !asset ? (
            <SkeletonChart />
          ) : (
            <>
              {/* Chart */}
              <div className="mb-6">
                <ChartContainer
                  asset={asset}
                  chartData={chartData}
                  selectedTimeframe={timeframe}
                  onTimeframeChange={handleTimeframeChange}
                  isLoading={isLoading}
                />
              </div>

              {/* Actions */}
              <div className="flex gap-3 mb-6">
                <Button
                  variant="secondary"
                  onClick={handleAddToWatchlist}
                >
                  <Star className="w-4 h-4" />
                  Add to Watchlist
                </Button>
                <Button
                  variant="secondary"
                  onClick={handleCreateAlert}
                >
                  <Bell className="w-4 h-4" />
                  Create Alert
                </Button>
              </div>

              {/* Market Stats */}
              <div className="bg-background-secondary border border-border-primary rounded p-6">
                <h2 className="text-xl font-semibold text-text-primary mb-4">
                  Market Stats
                </h2>

                <div className="grid grid-cols-2 md:grid-cols-4 gap-6">
                  <div>
                    <p className="text-text-tertiary text-sm mb-1">Open</p>
                    <p className="text-text-primary font-medium mono">
                      {formatCurrency(asset.open24h || 0)}
                    </p>
                  </div>

                  <div>
                    <p className="text-text-tertiary text-sm mb-1">High</p>
                    <p className="text-text-primary font-medium mono">
                      {formatCurrency(asset.high24h || 0)}
                    </p>
                  </div>

                  <div>
                    <p className="text-text-tertiary text-sm mb-1">Low</p>
                    <p className="text-text-primary font-medium mono">
                      {formatCurrency(asset.low24h || 0)}
                    </p>
                  </div>

                  <div>
                    <p className="text-text-tertiary text-sm mb-1">Volume</p>
                    <p className="text-text-primary font-medium mono">
                      {formatLargeNumber(asset.volume24h)}
                    </p>
                  </div>

                  <div>
                    <p className="text-text-tertiary text-sm mb-1">Market Cap</p>
                    <p className="text-text-primary font-medium mono">
                      {formatLargeNumber(asset.marketCap)}
                    </p>
                  </div>

                  <div>
                    <p className="text-text-tertiary text-sm mb-1">Type</p>
                    <Badge variant="neutral">
                      {ASSET_TYPE_LABELS[asset.type]}
                    </Badge>
                  </div>

                  <div className="col-span-2">
                    <p className="text-text-tertiary text-sm mb-1">Last Updated</p>
                    <p className="text-text-primary font-medium">
                      {formatDateTime(asset.lastUpdated)}
                    </p>
                  </div>
                </div>
              </div>
            </>
          )}
        </div>
      </main>

      {/* Add to Watchlist Modal */}
      {showWatchlistModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-background-secondary border border-border-primary rounded-lg p-6 w-full max-w-md mx-4">
            <h3 className="text-lg font-semibold text-text-primary mb-4">
              Add to Watchlist
            </h3>

            <select
              value={selectedWatchlist}
              onChange={(e) => setSelectedWatchlist(e.target.value)}
              className="w-full bg-background-tertiary border border-border-primary rounded px-3 py-2 text-text-primary mb-4"
            >
              <option value="">Select a watchlist</option>
              {watchlists.map((watchlist: Watchlist) => (
                <option key={watchlist.id} value={watchlist.id}>
                  {watchlist.name}
                </option>
              ))}
            </select>

            <div className="flex justify-end gap-3">
              <Button
                variant="ghost"
                onClick={() => {
                  setShowWatchlistModal(false)
                  setSelectedWatchlist('')
                }}
              >
                Cancel
              </Button>
              <Button
                variant="primary"
                onClick={handleConfirmAddToWatchlist}
                disabled={!selectedWatchlist}
              >
                Add
              </Button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
