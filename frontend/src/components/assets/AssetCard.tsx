/**
 * Asset Card Component
 * Displays asset information in a card format
 */

import React from 'react'
import { useNavigate } from 'react-router-dom'
import { Asset } from '@/types'
import { Card, Badge } from '@/components/common'
import { formatCurrency, formatPercent, formatLargeNumber, ASSET_TYPE_LABELS } from '@/utils'

export interface AssetCardProps {
  asset: Asset
  onClick?: (asset: Asset) => void
}

export function AssetCard({ asset, onClick }: AssetCardProps) {
  const navigate = useNavigate()

  const handleClick = () => {
    if (onClick) {
      onClick(asset)
    } else {
      navigate(`/asset/${asset.symbol}`)
    }
  }

  const priceChange = asset.changePercent24h || asset.change24h
  const isPositive = priceChange >= 0

  return (
    <Card
      hoverable
      onClick={handleClick}
      className="cursor-pointer"
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault()
          handleClick()
        }
      }}
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-3">
        <div className="flex-1 min-w-0">
          <h3 className="text-lg font-semibold text-text-primary truncate">
            {asset.symbol}
          </h3>
          <p className="text-sm text-text-secondary truncate">{asset.name}</p>
        </div>
        <Badge variant="neutral" size="sm">
          {ASSET_TYPE_LABELS[asset.type]}
        </Badge>
      </div>

      {/* Price */}
      <div className="mb-3">
        <p className="text-2xl font-semibold text-text-primary mono">
          {formatCurrency(asset.price)}
        </p>
        <p className={`text-sm font-medium ${isPositive ? 'price-up' : 'price-down'}`}>
          {formatPercent(priceChange)}
        </p>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-2 gap-2 text-sm">
        <div>
          <p className="text-text-tertiary">Volume</p>
          <p className="text-text-primary font-medium mono">
            {formatLargeNumber(asset.volume24h)}
          </p>
        </div>
        <div>
          <p className="text-text-tertiary">Market Cap</p>
          <p className="text-text-primary font-medium mono">
            {formatLargeNumber(asset.marketCap)}
          </p>
        </div>
      </div>
    </Card>
  )
}
