/**
 * Authentication Context
 * Manages user authentication state and provides auth functions
 */

import { createContext, useContext, useState, useEffect, useCallback } from 'react'
import type {
  LoginCredentials,
  RegisterData,
  AuthResponse,
  AuthState,
} from '@/types'
import { LOCAL_STORAGE_KEYS } from '@/utils'
import * as authService from '@/services/auth.service'

interface AuthContextValue extends AuthState {
  login: (credentials: LoginCredentials) => Promise<void>
  register: (data: RegisterData) => Promise<void>
  logout: () => Promise<void>
  refreshUser: () => Promise<void>
}

const AuthContext = createContext<AuthContextValue | undefined>(undefined)

interface AuthProviderProps {
  children: React.ReactNode
}

export function AuthProvider({ children }: AuthProviderProps) {
  const [state, setState] = useState<AuthState>({
    user: null,
    token: null,
    isAuthenticated: false,
    isLoading: true,
    error: null,
  })

  // Check authentication on mount
  useEffect(() => {
    const checkAuth = async () => {
      const token = localStorage.getItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN)

      // No token - not authenticated
      if (!token) {
        setState({
          user: null,
          token: null,
          isAuthenticated: false,
          isLoading: false,
          error: null,
        })
        return
      }

      try {
        // Try to verify token by fetching current user from API
        const isValid = await authService.verifyToken()

        if (isValid) {
          const user = authService.getCurrentUser()

          setState({
            user,
            token,
            isAuthenticated: true,
            isLoading: false,
            error: null,
          })
        } else {
          // Token invalid - try to refresh
          await tryRefreshToken()
        }
      } catch (error) {
        // Verification failed - try refresh token
        try {
          await tryRefreshToken()
        } catch {
          // Refresh also failed - clear auth state
          authService.clearAuthStorage()
          setState({
            user: null,
            token: null,
            isAuthenticated: false,
            isLoading: false,
            error: null,
          })
        }
      }
    }

    const tryRefreshToken = async () => {
      try {
        const newToken = await authService.refreshToken()
        const user = authService.getCurrentUser()

        setState({
          user,
          token: newToken,
          isAuthenticated: true,
          isLoading: false,
          error: null,
        })
      } catch {
        // Refresh failed - clear auth state
        authService.clearAuthStorage()
        setState({
          user: null,
          token: null,
          isAuthenticated: false,
          isLoading: false,
          error: null,
        })
      }
    }

    checkAuth()
  }, [])

  const login = useCallback(async (credentials: LoginCredentials) => {
    try {
      setState((prev) => ({ ...prev, isLoading: true, error: null }))

      const response: AuthResponse = await authService.login(credentials)

      setState({
        user: response.user,
        token: response.token,
        isAuthenticated: true,
        isLoading: false,
        error: null,
      })
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : 'Login failed'

      setState({
        user: null,
        token: null,
        isAuthenticated: false,
        isLoading: false,
        error: errorMessage,
      })

      throw error
    }
  }, [])

  const register = useCallback(async (data: RegisterData) => {
    try {
      setState((prev) => ({ ...prev, isLoading: true, error: null }))

      const response: AuthResponse = await authService.register(data)

      setState({
        user: response.user,
        token: response.token,
        isAuthenticated: true,
        isLoading: false,
        error: null,
      })
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : 'Registration failed'

      setState({
        user: null,
        token: null,
        isAuthenticated: false,
        isLoading: false,
        error: errorMessage,
      })

      throw error
    }
  }, [])

  const logout = useCallback(async () => {
    try {
      await authService.logout()
    } finally {
      setState({
        user: null,
        token: null,
        isAuthenticated: false,
        isLoading: false,
        error: null,
      })
    }
  }, [])

  const refreshUser = useCallback(async () => {
    try {
      const user = await authService.fetchCurrentUser()
      setState((prev) => ({ ...prev, user }))
    } catch (error) {
      console.error('Failed to refresh user:', error)
    }
  }, [])

  const value: AuthContextValue = {
    ...state,
    login,
    register,
    logout,
    refreshUser,
  }

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}

export function useAuth(): AuthContextValue {
  const context = useContext(AuthContext)

  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider')
  }

  return context
}
