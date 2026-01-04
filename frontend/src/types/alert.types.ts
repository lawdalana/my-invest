/**
 * Alert Type Definitions
 * Types for price and pattern alerts
 */

import { AssetType } from './asset.types'

export type AlertType = 'price' | 'pattern'

export type PriceCondition = 'above' | 'below' | 'crosses_above' | 'crosses_below'

export type PatternType =
  | 'double_top'
  | 'double_bottom'
  | 'head_shoulders'
  | 'inverse_head_shoulders'
  | 'triangle'
  | 'wedge'
  | 'flag'
  | 'pennant'

export type AlertStatus = 'active' | 'triggered' | 'expired' | 'disabled'

export interface Alert {
  id: string
  userId: string
  symbol: string
  assetName: string
  assetType: AssetType
  type: AlertType
  status: AlertStatus
  createdAt: string
  updatedAt: string
  triggeredAt?: string
  expiresAt?: string
  notificationSent: boolean

  // Price Alert Fields
  priceCondition?: PriceCondition
  targetPrice?: number

  // Pattern Alert Fields
  patternType?: PatternType
  timeframe?: string

  // Common Fields
  message?: string
  oneTime: boolean // If true, delete after triggering
  emailNotification: boolean
  pushNotification: boolean
}

export interface CreatePriceAlertDto {
  symbol: string
  assetName: string
  assetType: AssetType
  priceCondition: PriceCondition
  targetPrice: number
  message?: string
  oneTime?: boolean
  emailNotification?: boolean
  pushNotification?: boolean
  expiresAt?: string
}

export interface CreatePatternAlertDto {
  symbol: string
  assetName: string
  assetType: AssetType
  patternType: PatternType
  timeframe: string
  message?: string
  oneTime?: boolean
  emailNotification?: boolean
  pushNotification?: boolean
  expiresAt?: string
}

export type CreateAlertDto = CreatePriceAlertDto | CreatePatternAlertDto

export interface UpdateAlertDto {
  status?: AlertStatus
  targetPrice?: number
  message?: string
  emailNotification?: boolean
  pushNotification?: boolean
  expiresAt?: string
}

export interface AlertSummary {
  total: number
  active: number
  triggered: number
  expired: number
  disabled: number
}

export interface AlertNotification {
  alertId: string
  symbol: string
  assetName: string
  type: AlertType
  message: string
  timestamp: string
  currentPrice?: number
  targetPrice?: number
  patternType?: PatternType
}
