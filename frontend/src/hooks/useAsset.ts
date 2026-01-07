/**
 * useAsset Hook
 * Fetches asset details and chart data with loading/error states
 */

import { useState, useEffect, useCallback } from 'react'
import type { Asset, ChartDataPoint, Timeframe } from '@/types'
import * as assetService from '@/services/asset.service'
import { getErrorMessage } from '@/services/api/client'

interface UseAssetResult {
  asset: Asset | null
  chartData: ChartDataPoint[]
  isLoading: boolean
  error: string | null
  timeframe: Timeframe
  setTimeframe: (timeframe: Timeframe) => void
  refetch: () => Promise<void>
}

export function useAsset(symbol: string | undefined): UseAssetResult {
  const [asset, setAsset] = useState<Asset | null>(null)
  const [chartData, setChartData] = useState<ChartDataPoint[]>([])
  const [timeframe, setTimeframe] = useState<Timeframe>('1D')
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchData = useCallback(async () => {
    if (!symbol) {
      setIsLoading(false)
      return
    }

    setIsLoading(true)
    setError(null)

    try {
      const [assetData, historyData] = await Promise.all([
        assetService.getAsset(symbol),
        assetService.getAssetHistory(symbol, timeframe),
      ])

      setAsset(assetData)
      setChartData(historyData)
    } catch (err) {
      const errorMessage = getErrorMessage(err)
      setError(errorMessage)
      console.error('Failed to fetch asset:', errorMessage)
    } finally {
      setIsLoading(false)
    }
  }, [symbol, timeframe])

  useEffect(() => {
    fetchData()
  }, [fetchData])

  const handleTimeframeChange = useCallback((newTimeframe: Timeframe) => {
    setTimeframe(newTimeframe)
  }, [])

  return {
    asset,
    chartData,
    isLoading,
    error,
    timeframe,
    setTimeframe: handleTimeframeChange,
    refetch: fetchData,
  }
}
