/**
 * Chart Container Component
 * TradingView Lightweight Charts integration with timeframe selector
 */

import React, { useEffect, useRef, useState } from 'react'
import { createChart, IChartApi, ISeriesApi, CandlestickData } from 'lightweight-charts'
import { Asset, ChartDataPoint, Timeframe } from '@/types'
import { PriceDisplay } from '@/components/assets/PriceDisplay'
import { Button } from '@/components/common'
import { SkeletonChart } from '@/components/common'
import { TIMEFRAMES, CHART_COLORS } from '@/utils'

export interface ChartContainerProps {
  asset: Asset
  chartData: ChartDataPoint[]
  selectedTimeframe: Timeframe
  onTimeframeChange: (timeframe: Timeframe) => void
  isLoading?: boolean
  className?: string
}

export function ChartContainer({
  asset,
  chartData,
  selectedTimeframe,
  onTimeframeChange,
  isLoading = false,
  className = '',
}: ChartContainerProps) {
  const chartContainerRef = useRef<HTMLDivElement>(null)
  const chartRef = useRef<IChartApi | null>(null)
  const seriesRef = useRef<ISeriesApi<'Candlestick'> | null>(null)
  const [chartHeight, setChartHeight] = useState(600)

  // Create chart
  useEffect(() => {
    if (!chartContainerRef.current || isLoading) return

    // Create chart instance
    const chart = createChart(chartContainerRef.current, {
      width: chartContainerRef.current.clientWidth,
      height: chartHeight,
      layout: {
        background: { color: CHART_COLORS.BACKGROUND },
        textColor: CHART_COLORS.TEXT,
      },
      grid: {
        vertLines: { color: CHART_COLORS.GRID },
        horzLines: { color: CHART_COLORS.GRID },
      },
      crosshair: {
        mode: 1, // Normal crosshair mode
        vertLine: {
          color: CHART_COLORS.CROSSHAIR,
          width: 1,
          style: 2, // Dashed
          labelBackgroundColor: CHART_COLORS.CROSSHAIR,
        },
        horzLine: {
          color: CHART_COLORS.CROSSHAIR,
          width: 1,
          style: 2, // Dashed
          labelBackgroundColor: CHART_COLORS.CROSSHAIR,
        },
      },
      timeScale: {
        timeVisible: true,
        secondsVisible: false,
        borderColor: CHART_COLORS.GRID,
      },
      rightPriceScale: {
        borderColor: CHART_COLORS.GRID,
      },
    })

    // Add candlestick series
    const candlestickSeries = chart.addCandlestickSeries({
      upColor: CHART_COLORS.UP,
      downColor: CHART_COLORS.DOWN,
      borderUpColor: CHART_COLORS.UP,
      borderDownColor: CHART_COLORS.DOWN,
      wickUpColor: CHART_COLORS.UP,
      wickDownColor: CHART_COLORS.DOWN,
    })

    chartRef.current = chart
    seriesRef.current = candlestickSeries

    // Handle resize
    const handleResize = () => {
      if (chartContainerRef.current && chart) {
        chart.applyOptions({
          width: chartContainerRef.current.clientWidth,
        })
      }
    }

    window.addEventListener('resize', handleResize)

    return () => {
      window.removeEventListener('resize', handleResize)
      chart.remove()
    }
  }, [chartHeight, isLoading])

  // Update chart data
  useEffect(() => {
    if (!seriesRef.current || !chartData || chartData.length === 0) return

    const candlestickData = chartData.map((point) => ({
      time: point.time,
      open: point.open,
      high: point.high,
      low: point.low,
      close: point.close,
    })) as CandlestickData[]

    seriesRef.current.setData(candlestickData)

    if (chartRef.current) {
      chartRef.current.timeScale().fitContent()
    }
  }, [chartData])

  // Responsive height
  useEffect(() => {
    const updateHeight = () => {
      if (window.innerWidth < 640) {
        setChartHeight(400)
      } else {
        setChartHeight(600)
      }
    }

    updateHeight()
    window.addEventListener('resize', updateHeight)

    return () => window.removeEventListener('resize', updateHeight)
  }, [])

  if (isLoading) {
    return <SkeletonChart />
  }

  return (
    <div className={`chart-container ${className}`}>
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 p-4 border-b border-border-primary">
        <div className="flex items-center gap-4">
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-xl font-semibold text-text-primary">
                {asset.symbol}
              </h2>
              <span className="text-sm text-text-tertiary">{asset.name}</span>
            </div>
            <PriceDisplay
              price={asset.price}
              change={asset.changePercent24h || asset.change24h}
              showChange
              size="md"
            />
          </div>
        </div>

        {/* Timeframe Selector */}
        <div className="flex gap-1 overflow-x-auto no-scrollbar">
          {TIMEFRAMES.map((timeframe) => (
            <Button
              key={timeframe}
              variant={selectedTimeframe === timeframe ? 'primary' : 'ghost'}
              size="sm"
              onClick={() => onTimeframeChange(timeframe)}
            >
              {timeframe}
            </Button>
          ))}
        </div>
      </div>

      {/* Chart */}
      <div ref={chartContainerRef} className="w-full" />
    </div>
  )
}
