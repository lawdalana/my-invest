/**
 * User Type Definitions
 * Types for authentication and user preferences
 */

export interface User {
  id: string
  email: string
  username?: string
  firstName?: string
  lastName?: string
  avatar?: string
  createdAt: string
  preferences: UserPreferences
}

export interface UserPreferences {
  theme: 'light' | 'dark'
  defaultTimeframe: string // '1D', '1W', etc.
  defaultAssetType: string // 'all', 'stock', 'crypto', etc.
  emailNotifications: boolean
  pushNotifications: boolean
  priceAlerts: boolean
  patternAlerts: boolean
  currency: string // 'USD', 'EUR', etc.
  language: string // 'en', 'es', etc.
}

export interface LoginCredentials {
  email: string
  password: string
  rememberMe?: boolean
}

export interface RegisterData {
  email: string
  password: string
  confirmPassword: string
  username?: string
  firstName?: string
  lastName?: string
}

export interface AuthResponse {
  user: User
  token: string
  refreshToken?: string
  expiresIn: number // Seconds until token expires
}

export interface AuthState {
  user: User | null
  token: string | null
  isAuthenticated: boolean
  isLoading: boolean
  error: string | null
}

export interface PasswordResetRequest {
  email: string
}

export interface PasswordReset {
  token: string
  password: string
  confirmPassword: string
}

export interface ChangePassword {
  currentPassword: string
  newPassword: string
  confirmPassword: string
}

export interface UpdateProfile {
  username?: string
  firstName?: string
  lastName?: string
  avatar?: string
}

export interface UpdatePreferences {
  theme?: 'light' | 'dark'
  defaultTimeframe?: string
  defaultAssetType?: string
  emailNotifications?: boolean
  pushNotifications?: boolean
  priceAlerts?: boolean
  patternAlerts?: boolean
  currency?: string
  language?: string
}
