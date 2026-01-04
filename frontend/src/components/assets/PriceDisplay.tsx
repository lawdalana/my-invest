/**
 * Price Display Component
 * Displays price with animated flash on change
 */

import React, { useEffect, useState, useRef } from 'react'
import { formatCurrency, formatPercent, formatPrice } from '@/utils'

export type PriceDisplaySize = 'sm' | 'md' | 'lg'

export interface PriceDisplayProps {
  price: number
  change?: number
  showChange?: boolean
  size?: PriceDisplaySize
  currency?: string
  decimals?: number
  className?: string
}

const sizeStyles: Record<PriceDisplaySize, { price: string; change: string }> = {
  sm: {
    price: 'text-lg',
    change: 'text-xs',
  },
  md: {
    price: 'text-2xl',
    change: 'text-sm',
  },
  lg: {
    price: 'text-3xl',
    change: 'text-base',
  },
}

export function PriceDisplay({
  price,
  change,
  showChange = false,
  size = 'md',
  currency = 'USD',
  decimals,
  className = '',
}: PriceDisplayProps) {
  const [flashClass, setFlashClass] = useState('')
  const prevPriceRef = useRef<number>(price)

  // Animate price change
  useEffect(() => {
    if (prevPriceRef.current !== undefined && prevPriceRef.current !== price) {
      const isUp = price > prevPriceRef.current
      setFlashClass(isUp ? 'flash-up' : 'flash-down')

      const timer = setTimeout(() => {
        setFlashClass('')
      }, 500)

      prevPriceRef.current = price

      return () => clearTimeout(timer)
    } else {
      prevPriceRef.current = price
    }
  }, [price])

  const formattedPrice = decimals !== undefined
    ? formatCurrency(price, currency, decimals)
    : formatPrice(price)

  const isPositiveChange = change !== undefined && change >= 0

  return (
    <div className={`flex flex-col gap-1 ${className}`}>
      <p
        className={`
          font-semibold text-text-primary mono
          ${sizeStyles[size].price}
          ${flashClass}
          transition-all duration-200
        `}
      >
        {currency === 'USD' && formattedPrice.startsWith('$')
          ? formattedPrice
          : formatCurrency(price, currency, decimals ?? 2)}
      </p>

      {showChange && change !== undefined && (
        <p
          className={`
            font-medium
            ${sizeStyles[size].change}
            ${isPositiveChange ? 'price-up' : 'price-down'}
          `}
        >
          {formatPercent(change)}
        </p>
      )}
    </div>
  )
}
