/**
 * Authentication Service
 * Handles user authentication with the backend API
 */

import apiClient from './api/client'
import { API_ENDPOINTS, LOCAL_STORAGE_KEYS } from '@/utils'
import {
  mapAuthResponse,
  mapUserResponse,
  type BackendAuthResponse,
  type BackendUserResponse,
} from '@/utils/mappers'
import type {
  User,
  LoginCredentials,
  RegisterData,
  AuthResponse,
} from '@/types'

/**
 * Login user with email and password
 */
export async function login(credentials: LoginCredentials): Promise<AuthResponse> {
  const response = await apiClient.post<BackendAuthResponse>(API_ENDPOINTS.LOGIN, {
    email: credentials.email,
    password: credentials.password,
  })

  const authData = mapAuthResponse(response.data)

  // Store tokens and user data
  localStorage.setItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN, authData.token)
  if (authData.refreshToken) {
    localStorage.setItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN, authData.refreshToken)
  }
  localStorage.setItem(LOCAL_STORAGE_KEYS.USER_DATA, JSON.stringify(authData.user))

  return authData
}

/**
 * Register new user
 */
export async function register(data: RegisterData): Promise<AuthResponse> {
  const response = await apiClient.post<BackendAuthResponse>(API_ENDPOINTS.REGISTER, {
    email: data.email,
    password: data.password,
    password_confirm: data.confirmPassword, // Backend expects password_confirm
  })

  const authData = mapAuthResponse(response.data)

  // Store tokens and user data
  localStorage.setItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN, authData.token)
  if (authData.refreshToken) {
    localStorage.setItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN, authData.refreshToken)
  }
  localStorage.setItem(LOCAL_STORAGE_KEYS.USER_DATA, JSON.stringify(authData.user))

  return authData
}

/**
 * Logout user
 */
export async function logout(): Promise<void> {
  const refreshTokenValue = localStorage.getItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN)

  try {
    // Try to invalidate refresh token on server
    if (refreshTokenValue) {
      await apiClient.post(API_ENDPOINTS.LOGOUT, {
        refresh_token: refreshTokenValue,
      })
    }
  } catch (error) {
    // Ignore logout errors - we'll clear local storage anyway
    console.warn('Logout request failed:', error)
  } finally {
    // Always clear local storage
    clearAuthStorage()
  }
}

/**
 * Get current user from backend API
 */
export async function fetchCurrentUser(): Promise<User> {
  const response = await apiClient.get<BackendUserResponse>(API_ENDPOINTS.CURRENT_USER)
  const user = mapUserResponse(response.data)

  // Update stored user data
  localStorage.setItem(LOCAL_STORAGE_KEYS.USER_DATA, JSON.stringify(user))

  return user
}

/**
 * Get current user from localStorage (without API call)
 * Kept for backwards compatibility with existing code
 */
export function getCurrentUser(): User | null {
  const userData = localStorage.getItem(LOCAL_STORAGE_KEYS.USER_DATA)
  return userData ? JSON.parse(userData) : null
}

/**
 * Check if user is authenticated (has token in localStorage)
 */
export function isAuthenticated(): boolean {
  const token = localStorage.getItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN)
  return !!token
}

/**
 * Verify token by fetching current user from API
 * Returns true if token is valid, false otherwise
 */
export async function verifyToken(): Promise<boolean> {
  if (!isAuthenticated()) {
    return false
  }

  try {
    await fetchCurrentUser()
    return true
  } catch {
    return false
  }
}

/**
 * Refresh access token using refresh token
 */
export async function refreshToken(): Promise<string> {
  const currentRefreshToken = localStorage.getItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN)

  if (!currentRefreshToken) {
    throw new Error('No refresh token available')
  }

  const response = await apiClient.post<BackendAuthResponse>(API_ENDPOINTS.REFRESH_TOKEN, {
    refresh_token: currentRefreshToken,
  })

  const authData = mapAuthResponse(response.data)

  // Update stored tokens
  localStorage.setItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN, authData.token)
  if (authData.refreshToken) {
    localStorage.setItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN, authData.refreshToken)
  }
  localStorage.setItem(LOCAL_STORAGE_KEYS.USER_DATA, JSON.stringify(authData.user))

  return authData.token
}

/**
 * Get stored auth token
 */
export function getAuthToken(): string | null {
  return localStorage.getItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN)
}

/**
 * Get stored refresh token
 */
export function getRefreshToken(): string | null {
  return localStorage.getItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN)
}

/**
 * Clear all auth data from localStorage
 */
export function clearAuthStorage(): void {
  localStorage.removeItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN)
  localStorage.removeItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN)
  localStorage.removeItem(LOCAL_STORAGE_KEYS.USER_DATA)
}
