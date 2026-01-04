/**
 * Asset Detail Page
 * Detailed asset view with chart and stats
 */

import React, { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { ArrowLeft, Star, Bell } from 'lucide-react'
import { Navigation, ChartContainer, Button, Badge, SkeletonChart } from '@/components'
import { Asset, ChartDataPoint, Timeframe } from '@/types'
import { formatCurrency, formatLargeNumber, formatDateTime, ASSET_TYPE_LABELS } from '@/utils'
import { useNotification } from '@/contexts'

// Mock data
const MOCK_ASSET: Asset = {
  id: '1',
  symbol: 'AAPL',
  name: 'Apple Inc.',
  type: 'stock',
  price: 178.25,
  change24h: 2.34,
  changePercent24h: 2.34,
  volume24h: 52840000,
  marketCap: 2890000000000,
  high24h: 180.50,
  low24h: 175.20,
  open24h: 176.00,
  lastUpdated: new Date().toISOString(),
}

const MOCK_CHART_DATA: ChartDataPoint[] = [
  { time: 1704067200, open: 176, high: 178, low: 175, close: 177 },
  { time: 1704153600, open: 177, high: 179, low: 176.5, close: 178.5 },
  { time: 1704240000, open: 178.5, high: 180, low: 177, close: 178.25 },
]

export function AssetDetailPage() {
  const { symbol } = useParams<{ symbol: string }>()
  const navigate = useNavigate()
  const { showToast } = useNotification()

  const [asset, setAsset] = useState<Asset | null>(null)
  const [chartData, setChartData] = useState<ChartDataPoint[]>([])
  const [selectedTimeframe, setSelectedTimeframe] = useState<Timeframe>('1D')
  const [isLoading, setIsLoading] = useState(true)

  // Load asset data
  useEffect(() => {
    const loadAsset = async () => {
      setIsLoading(true)

      // Simulate API call
      await new Promise((resolve) => setTimeout(resolve, 1000))

      setAsset(MOCK_ASSET)
      setChartData(MOCK_CHART_DATA)
      setIsLoading(false)
    }

    loadAsset()
  }, [symbol])

  const handleTimeframeChange = async (timeframe: Timeframe) => {
    setSelectedTimeframe(timeframe)

    // Simulate loading new chart data
    setIsLoading(true)
    await new Promise((resolve) => setTimeout(resolve, 500))
    setChartData(MOCK_CHART_DATA)
    setIsLoading(false)
  }

  const handleAddToWatchlist = () => {
    showToast('Added to watchlist', 'success')
  }

  const handleCreateAlert = () => {
    showToast('Alert creation coming soon', 'info')
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
                  selectedTimeframe={selectedTimeframe}
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
    </div>
  )
}
