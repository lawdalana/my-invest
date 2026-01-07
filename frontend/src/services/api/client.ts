/**
 * API Client
 * Axios instance with request/response interceptors for authentication and error handling
 */

import axios, { AxiosInstance, AxiosError, InternalAxiosRequestConfig } from 'axios'
import { ApiError, ApiResponse } from '@/types'
import { LOCAL_STORAGE_KEYS, API_ENDPOINTS } from '@/utils'

// Flag to prevent multiple simultaneous refresh attempts
let isRefreshing = false
let failedQueue: Array<{
  resolve: (value: unknown) => void
  reject: (reason?: unknown) => void
}> = []

const processQueue = (error: Error | null, token: string | null = null) => {
  failedQueue.forEach((prom) => {
    if (error) {
      prom.reject(error)
    } else {
      prom.resolve(token)
    }
  })
  failedQueue = []
}

// Create axios instance with base configuration
const apiClient: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080/api',
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
})

/**
 * Request Interceptor
 * Adds authentication token to all requests
 */
apiClient.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    const token = localStorage.getItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN)

    if (token && config.headers) {
      config.headers.Authorization = `Bearer ${token}`
    }

    return config
  },
  (error: AxiosError) => {
    return Promise.reject(error)
  }
)

/**
 * Response Interceptor
 * Handles authentication errors with automatic token refresh
 */
apiClient.interceptors.response.use(
  (response) => {
    return response
  },
  async (error: AxiosError) => {
    const originalRequest = error.config as InternalAxiosRequestConfig & { _retry?: boolean }
    const status = error.response?.status

    // Handle 401 Unauthorized - try token refresh
    if (status === 401 && !originalRequest._retry) {
      // Don't retry for auth endpoints (login, register, refresh)
      const isAuthEndpoint = originalRequest.url?.includes('/auth/')

      if (isAuthEndpoint) {
        return Promise.reject(error)
      }

      if (isRefreshing) {
        // If already refreshing, queue this request
        return new Promise((resolve, reject) => {
          failedQueue.push({ resolve, reject })
        })
          .then((token) => {
            if (originalRequest.headers) {
              originalRequest.headers.Authorization = `Bearer ${token}`
            }
            return apiClient(originalRequest)
          })
          .catch((err) => Promise.reject(err))
      }

      originalRequest._retry = true
      isRefreshing = true

      const refreshToken = localStorage.getItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN)

      if (!refreshToken) {
        // No refresh token - clear auth and redirect
        clearAuthAndRedirect()
        return Promise.reject(error)
      }

      try {
        // Try to refresh the token
        const response = await axios.post(
          `${apiClient.defaults.baseURL}${API_ENDPOINTS.REFRESH_TOKEN}`,
          { refresh_token: refreshToken }
        )

        const newToken = response.data.access_token
        const newRefreshToken = response.data.refresh_token

        // Store new tokens
        localStorage.setItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN, newToken)
        if (newRefreshToken) {
          localStorage.setItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN, newRefreshToken)
        }

        // Update user data if returned
        if (response.data.user) {
          localStorage.setItem(LOCAL_STORAGE_KEYS.USER_DATA, JSON.stringify(response.data.user))
        }

        // Update authorization header
        if (originalRequest.headers) {
          originalRequest.headers.Authorization = `Bearer ${newToken}`
        }

        processQueue(null, newToken)

        return apiClient(originalRequest)
      } catch (refreshError) {
        processQueue(refreshError as Error, null)
        clearAuthAndRedirect()
        return Promise.reject(refreshError)
      } finally {
        isRefreshing = false
      }
    }

    // Handle 429 Too Many Requests
    if (status === 429) {
      const retryAfter = error.response?.headers['retry-after']
      console.error(`Rate limit exceeded. Retry after ${retryAfter || 'unknown'} seconds.`)
    }

    // Handle 500 Internal Server Error
    if (status === 500) {
      console.error('Internal server error. Please try again later.')
    }

    // Handle network errors
    if (!error.response) {
      console.error('Network error. Please check your connection.')
    }

    return Promise.reject(error)
  }
)

/**
 * Clear authentication data and redirect to login
 */
function clearAuthAndRedirect(): void {
  localStorage.removeItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN)
  localStorage.removeItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN)
  localStorage.removeItem(LOCAL_STORAGE_KEYS.USER_DATA)

  // Only redirect if not already on login page
  if (window.location.pathname !== '/login' && window.location.pathname !== '/register') {
    window.location.href = '/login'
  }
}

/**
 * Helper function to extract error message from API error response
 */
export function getErrorMessage(error: unknown): string {
  if (axios.isAxiosError(error)) {
    const apiError = error.response?.data as ApiError | undefined

    // Check for backend error format
    if (apiError?.error?.message) {
      return apiError.error.message
    }

    // Check for direct message in response
    if (error.response?.data?.message) {
      return error.response.data.message as string
    }

    if (error.response?.statusText) {
      return error.response.statusText
    }

    if (error.message) {
      return error.message
    }
  }

  if (error instanceof Error) {
    return error.message
  }

  return 'An unexpected error occurred'
}

/**
 * Type-safe wrapper for API responses
 */
export async function apiRequest<T>(
  request: Promise<{ data: ApiResponse<T> }>
): Promise<T> {
  try {
    const response = await request
    return response.data.data
  } catch (error) {
    throw new Error(getErrorMessage(error))
  }
}

export default apiClient
