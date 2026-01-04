/**
 * API Client
 * Axios instance with request/response interceptors for authentication and error handling
 */

import axios, { AxiosInstance, AxiosError, InternalAxiosRequestConfig } from 'axios'
import { ApiError, ApiResponse } from '@/types'
import { LOCAL_STORAGE_KEYS } from '@/utils'

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
 * Handles authentication errors and redirects to login
 */
apiClient.interceptors.response.use(
  (response) => {
    return response
  },
  async (error: AxiosError) => {
    const status = error.response?.status

    // Handle 401 Unauthorized - redirect to login
    if (status === 401) {
      // Clear authentication data
      localStorage.removeItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN)
      localStorage.removeItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN)
      localStorage.removeItem(LOCAL_STORAGE_KEYS.USER_DATA)

      // Redirect to login page
      if (window.location.pathname !== '/login') {
        window.location.href = '/login'
      }
    }

    // Handle 429 Too Many Requests
    if (status === 429) {
      console.error('Rate limit exceeded. Please try again later.')
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
 * Helper function to extract error message from API error response
 */
export function getErrorMessage(error: unknown): string {
  if (axios.isAxiosError(error)) {
    const apiError = error.response?.data as ApiError | undefined

    if (apiError?.error?.message) {
      return apiError.error.message
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
