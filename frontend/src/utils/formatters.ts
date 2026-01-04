/**
 * Formatting Utilities
 * Helper functions for formatting numbers, currencies, dates, and percentages
 */

/**
 * Format a number as currency with proper symbol and decimal places
 * @param value - Number to format
 * @param currency - Currency code (default: 'USD')
 * @param decimals - Number of decimal places (default: 2)
 * @returns Formatted currency string
 */
export function formatCurrency(
  value: number,
  currency: string = 'USD',
  decimals: number = 2
): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency,
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(value)
}

/**
 * Format a percentage with + or - sign and % symbol
 * @param value - Percentage value (e.g., 5.23 for 5.23%)
 * @param decimals - Number of decimal places (default: 2)
 * @returns Formatted percentage string with sign
 */
export function formatPercent(value: number, decimals: number = 2): string {
  const sign = value >= 0 ? '+' : ''
  return `${sign}${value.toFixed(decimals)}%`
}

/**
 * Format large numbers with K, M, B, T suffixes
 * @param value - Number to format
 * @param decimals - Number of decimal places (default: 1)
 * @returns Formatted number string with suffix
 */
export function formatLargeNumber(
  value: number,
  decimals: number = 1
): string {
  if (value === 0) return '0'

  const abs = Math.abs(value)
  const sign = value < 0 ? '-' : ''

  if (abs >= 1e12) {
    return `${sign}${(abs / 1e12).toFixed(decimals)}T`
  }
  if (abs >= 1e9) {
    return `${sign}${(abs / 1e9).toFixed(decimals)}B`
  }
  if (abs >= 1e6) {
    return `${sign}${(abs / 1e6).toFixed(decimals)}M`
  }
  if (abs >= 1e3) {
    return `${sign}${(abs / 1e3).toFixed(decimals)}K`
  }

  return value.toFixed(decimals)
}

/**
 * Format a date as a readable string
 * @param date - Date to format (string, number, or Date object)
 * @param options - Intl.DateTimeFormat options
 * @returns Formatted date string
 */
export function formatDate(
  date: string | number | Date,
  options: Intl.DateTimeFormatOptions = {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  }
): string {
  const dateObj = typeof date === 'string' ? new Date(date) : date
  return new Intl.DateTimeFormat('en-US', options).format(dateObj as Date)
}

/**
 * Format a date and time as a readable string
 * @param date - Date to format (string, number, or Date object)
 * @returns Formatted date and time string
 */
export function formatDateTime(date: string | number | Date): string {
  return formatDate(date, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

/**
 * Format a date as relative time (e.g., "2 hours ago", "3 days ago")
 * @param date - Date to format (string, number, or Date object)
 * @returns Relative time string
 */
export function formatRelativeTime(date: string | number | Date): string {
  const dateObj = typeof date === 'string' ? new Date(date) : new Date(date)
  const now = new Date()
  const diffMs = now.getTime() - dateObj.getTime()
  const diffSec = Math.floor(diffMs / 1000)
  const diffMin = Math.floor(diffSec / 60)
  const diffHour = Math.floor(diffMin / 60)
  const diffDay = Math.floor(diffHour / 24)
  const diffWeek = Math.floor(diffDay / 7)
  const diffMonth = Math.floor(diffDay / 30)
  const diffYear = Math.floor(diffDay / 365)

  if (diffSec < 60) {
    return 'just now'
  } else if (diffMin < 60) {
    return `${diffMin} minute${diffMin !== 1 ? 's' : ''} ago`
  } else if (diffHour < 24) {
    return `${diffHour} hour${diffHour !== 1 ? 's' : ''} ago`
  } else if (diffDay < 7) {
    return `${diffDay} day${diffDay !== 1 ? 's' : ''} ago`
  } else if (diffWeek < 4) {
    return `${diffWeek} week${diffWeek !== 1 ? 's' : ''} ago`
  } else if (diffMonth < 12) {
    return `${diffMonth} month${diffMonth !== 1 ? 's' : ''} ago`
  } else {
    return `${diffYear} year${diffYear !== 1 ? 's' : ''} ago`
  }
}

/**
 * Format a number with proper decimal places based on value
 * @param value - Number to format
 * @returns Formatted number string
 */
export function formatPrice(value: number): string {
  if (value >= 1000) {
    return value.toFixed(2)
  } else if (value >= 1) {
    return value.toFixed(4)
  } else if (value >= 0.01) {
    return value.toFixed(6)
  } else {
    return value.toFixed(8)
  }
}

/**
 * Truncate a string to a maximum length with ellipsis
 * @param str - String to truncate
 * @param maxLength - Maximum length
 * @returns Truncated string
 */
export function truncate(str: string, maxLength: number): string {
  if (str.length <= maxLength) return str
  return str.slice(0, maxLength - 3) + '...'
}
