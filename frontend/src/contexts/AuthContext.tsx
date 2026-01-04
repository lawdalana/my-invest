/**
 * Authentication Context
 * Manages user authentication state and provides auth functions
 */

import { createContext, useContext, useState, useEffect } from 'react'
import type {
  LoginCredentials,
  RegisterData,
  AuthResponse,
  AuthState,
} from '@/types'
import * as authService from '@/services/auth.service'

interface AuthContextValue extends AuthState {
  login: (credentials: LoginCredentials) => Promise<void>
  register: (data: RegisterData) => Promise<void>
  logout: () => void
  refreshUser: () => void
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
      try {
        const isValid = await authService.verifyToken()

        if (isValid) {
          const user = authService.getCurrentUser()
          const token = localStorage.getItem('my-invest-auth-token')

          setState({
            user,
            token,
            isAuthenticated: true,
            isLoading: false,
            error: null,
          })
        } else {
          setState({
            user: null,
            token: null,
            isAuthenticated: false,
            isLoading: false,
            error: null,
          })
        }
      } catch (error) {
        setState({
          user: null,
          token: null,
          isAuthenticated: false,
          isLoading: false,
          error: error instanceof Error ? error.message : 'Auth check failed',
        })
      }
    }

    checkAuth()
  }, [])

  const login = async (credentials: LoginCredentials) => {
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
  }

  const register = async (data: RegisterData) => {
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
  }

  const logout = () => {
    authService.logout()
    setState({
      user: null,
      token: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,
    })
  }

  const refreshUser = () => {
    const user = authService.getCurrentUser()
    setState((prev) => ({ ...prev, user }))
  }

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
