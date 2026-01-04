/**
 * Validation Utilities
 * Helper functions for validating user input
 */

/**
 * Validate email address format
 * @param email - Email address to validate
 * @returns True if email is valid
 */
export function validateEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email)
}

/**
 * Validate password strength
 * Requirements:
 * - At least 8 characters
 * - At least one uppercase letter
 * - At least one lowercase letter
 * - At least one number
 * - At least one special character
 * @param password - Password to validate
 * @returns Object with validation result and errors
 */
export function validatePassword(password: string): {
  isValid: boolean
  errors: string[]
  strength: 'weak' | 'medium' | 'strong'
} {
  const errors: string[] = []
  let strength: 'weak' | 'medium' | 'strong' = 'weak'

  if (password.length < 8) {
    errors.push('Password must be at least 8 characters long')
  }

  if (!/[A-Z]/.test(password)) {
    errors.push('Password must contain at least one uppercase letter')
  }

  if (!/[a-z]/.test(password)) {
    errors.push('Password must contain at least one lowercase letter')
  }

  if (!/\d/.test(password)) {
    errors.push('Password must contain at least one number')
  }

  if (!/[!@#$%^&*()_+\-=[\]{};':"\\|,.<>/?]/.test(password)) {
    errors.push('Password must contain at least one special character')
  }

  // Determine strength
  if (errors.length === 0) {
    if (password.length >= 12) {
      strength = 'strong'
    } else {
      strength = 'medium'
    }
  }

  return {
    isValid: errors.length === 0,
    errors,
    strength,
  }
}

/**
 * Validate stock/crypto symbol format
 * @param symbol - Symbol to validate
 * @returns True if symbol is valid
 */
export function validateSymbol(symbol: string): boolean {
  // Allow alphanumeric characters and some special characters (e.g., BTC-USD)
  const symbolRegex = /^[A-Z0-9.-]{1,10}$/i
  return symbolRegex.test(symbol)
}

/**
 * Validate number is positive
 * @param value - Number to validate
 * @returns True if number is positive
 */
export function isPositiveNumber(value: number): boolean {
  return !isNaN(value) && value > 0
}

/**
 * Validate URL format
 * @param url - URL to validate
 * @returns True if URL is valid
 */
export function validateUrl(url: string): boolean {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

/**
 * Validate required field is not empty
 * @param value - Value to validate
 * @returns True if value is not empty
 */
export function isRequired(value: string | number | null | undefined): boolean {
  if (value === null || value === undefined) return false
  if (typeof value === 'string') return value.trim().length > 0
  return true
}

/**
 * Validate string length is within range
 * @param value - String to validate
 * @param min - Minimum length
 * @param max - Maximum length
 * @returns True if length is within range
 */
export function validateLength(
  value: string,
  min: number,
  max: number
): boolean {
  const length = value.trim().length
  return length >= min && length <= max
}

/**
 * Validate number is within range
 * @param value - Number to validate
 * @param min - Minimum value
 * @param max - Maximum value
 * @returns True if value is within range
 */
export function validateRange(
  value: number,
  min: number,
  max: number
): boolean {
  return !isNaN(value) && value >= min && value <= max
}

/**
 * Check if two passwords match
 * @param password - First password
 * @param confirmPassword - Second password
 * @returns True if passwords match
 */
export function passwordsMatch(
  password: string,
  confirmPassword: string
): boolean {
  return password === confirmPassword && password.length > 0
}
