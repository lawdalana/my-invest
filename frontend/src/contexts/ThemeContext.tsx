/**
 * Theme Context
 * Manages theme state (dark/light mode)
 */

import React, { createContext, useContext, useState, useEffect } from 'react'
import { LOCAL_STORAGE_KEYS } from '@/utils'

type Theme = 'light' | 'dark'

interface ThemeContextValue {
  theme: Theme
  setTheme: (theme: Theme) => void
  toggleTheme: () => void
}

const ThemeContext = createContext<ThemeContextValue | undefined>(undefined)

interface ThemeProviderProps {
  children: React.ReactNode
  defaultTheme?: Theme
}

export function ThemeProvider({
  children,
  defaultTheme = 'dark',
}: ThemeProviderProps) {
  const [theme, setThemeState] = useState<Theme>(() => {
    // Check localStorage first
    const stored = localStorage.getItem(LOCAL_STORAGE_KEYS.THEME) as Theme | null
    if (stored === 'light' || stored === 'dark') {
      return stored
    }

    // Check system preference
    if (window.matchMedia('(prefers-color-scheme: light)').matches) {
      return 'light'
    }

    return defaultTheme
  })

  // Update document root class and localStorage when theme changes
  useEffect(() => {
    const root = document.documentElement

    // Remove both classes first
    root.classList.remove('light', 'dark')

    // Add current theme class
    root.classList.add(theme)

    // Save to localStorage
    localStorage.setItem(LOCAL_STORAGE_KEYS.THEME, theme)
  }, [theme])

  // Listen for system theme changes
  useEffect(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')

    const handleChange = (e: MediaQueryListEvent) => {
      // Only update if user hasn't manually set a preference
      const stored = localStorage.getItem(LOCAL_STORAGE_KEYS.THEME)
      if (!stored) {
        setThemeState(e.matches ? 'dark' : 'light')
      }
    }

    mediaQuery.addEventListener('change', handleChange)

    return () => mediaQuery.removeEventListener('change', handleChange)
  }, [])

  const setTheme = (newTheme: Theme) => {
    setThemeState(newTheme)
  }

  const toggleTheme = () => {
    setThemeState((prev) => (prev === 'dark' ? 'light' : 'dark'))
  }

  const value: ThemeContextValue = {
    theme,
    setTheme,
    toggleTheme,
  }

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>
}

export function useTheme(): ThemeContextValue {
  const context = useContext(ThemeContext)

  if (context === undefined) {
    throw new Error('useTheme must be used within a ThemeProvider')
  }

  return context
}
