/**
 * Authentication Service
 * Mock authentication service for MVP (uses localStorage)
 * In production, this will be replaced with real API calls
 */

import {
  User,
  LoginCredentials,
  RegisterData,
  AuthResponse,
} from '@/types'
import { LOCAL_STORAGE_KEYS } from '@/utils'

// Mock user database (stored in localStorage)
interface MockUser {
  id: string
  email: string
  password: string // In real app, this would be hashed server-side
  username?: string
  firstName?: string
  lastName?: string
  createdAt: string
}

/**
 * Get mock users from localStorage
 */
function getMockUsers(): MockUser[] {
  const users = localStorage.getItem(LOCAL_STORAGE_KEYS.MOCK_USERS)
  return users ? JSON.parse(users) : []
}

/**
 * Save mock users to localStorage
 */
function saveMockUsers(users: MockUser[]): void {
  localStorage.setItem(LOCAL_STORAGE_KEYS.MOCK_USERS, JSON.stringify(users))
}

/**
 * Generate a mock JWT token
 */
function generateMockToken(userId: string): string {
  // In production, this would be a real JWT from the backend
  return btoa(JSON.stringify({ userId, exp: Date.now() + 24 * 60 * 60 * 1000 }))
}

/**
 * Convert MockUser to User
 */
function mockUserToUser(mockUser: MockUser): User {
  return {
    id: mockUser.id,
    email: mockUser.email,
    username: mockUser.username,
    firstName: mockUser.firstName,
    lastName: mockUser.lastName,
    createdAt: mockUser.createdAt,
    preferences: {
      theme: 'dark',
      defaultTimeframe: '1D',
      defaultAssetType: 'all',
      emailNotifications: true,
      pushNotifications: false,
      priceAlerts: true,
      patternAlerts: true,
      currency: 'USD',
      language: 'en',
    },
  }
}

/**
 * Login user
 */
export async function login(
  credentials: LoginCredentials
): Promise<AuthResponse> {
  // Simulate API delay
  await new Promise((resolve) => setTimeout(resolve, 500))

  const users = getMockUsers()
  const user = users.find((u) => u.email === credentials.email)

  if (!user || user.password !== credentials.password) {
    throw new Error('Invalid email or password')
  }

  const token = generateMockToken(user.id)
  const userData = mockUserToUser(user)

  // Store token and user data
  localStorage.setItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN, token)
  localStorage.setItem(LOCAL_STORAGE_KEYS.USER_DATA, JSON.stringify(userData))

  if (credentials.rememberMe) {
    localStorage.setItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN, token)
  }

  return {
    user: userData,
    token,
    expiresIn: 86400, // 24 hours
  }
}

/**
 * Register new user
 */
export async function register(data: RegisterData): Promise<AuthResponse> {
  // Simulate API delay
  await new Promise((resolve) => setTimeout(resolve, 500))

  const users = getMockUsers()

  // Check if user already exists
  if (users.some((u) => u.email === data.email)) {
    throw new Error('User with this email already exists')
  }

  // Check if passwords match
  if (data.password !== data.confirmPassword) {
    throw new Error('Passwords do not match')
  }

  // Create new user
  const newUser: MockUser = {
    id: Date.now().toString(),
    email: data.email,
    password: data.password,
    username: data.username,
    firstName: data.firstName,
    lastName: data.lastName,
    createdAt: new Date().toISOString(),
  }

  // Save user
  users.push(newUser)
  saveMockUsers(users)

  // Auto-login after registration
  return login({
    email: data.email,
    password: data.password,
    rememberMe: true,
  })
}

/**
 * Logout user
 */
export function logout(): void {
  localStorage.removeItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN)
  localStorage.removeItem(LOCAL_STORAGE_KEYS.REFRESH_TOKEN)
  localStorage.removeItem(LOCAL_STORAGE_KEYS.USER_DATA)
}

/**
 * Get current user from localStorage
 */
export function getCurrentUser(): User | null {
  const userData = localStorage.getItem(LOCAL_STORAGE_KEYS.USER_DATA)
  return userData ? JSON.parse(userData) : null
}

/**
 * Check if user is authenticated
 */
export function isAuthenticated(): boolean {
  const token = localStorage.getItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN)
  const user = getCurrentUser()
  return !!(token && user)
}

/**
 * Verify token validity
 * In production, this would make an API call to verify the token
 */
export async function verifyToken(): Promise<boolean> {
  await new Promise((resolve) => setTimeout(resolve, 100))

  const token = localStorage.getItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN)
  if (!token) return false

  try {
    const decoded = JSON.parse(atob(token))
    return decoded.exp > Date.now()
  } catch {
    return false
  }
}

/**
 * Refresh token
 * In production, this would make an API call to get a new token
 */
export async function refreshToken(): Promise<string> {
  await new Promise((resolve) => setTimeout(resolve, 100))

  const user = getCurrentUser()
  if (!user) throw new Error('No user found')

  const newToken = generateMockToken(user.id)
  localStorage.setItem(LOCAL_STORAGE_KEYS.AUTH_TOKEN, newToken)

  return newToken
}
