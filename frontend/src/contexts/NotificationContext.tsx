/**
 * Notification Context
 * Manages toast notifications
 */

import React, { createContext, useContext, useState, useCallback } from 'react'

export type ToastType = 'success' | 'error' | 'warning' | 'info'

export interface Toast {
  id: string
  message: string
  type: ToastType
  duration?: number
}

interface NotificationContextValue {
  toasts: Toast[]
  showToast: (message: string, type?: ToastType, duration?: number) => void
  removeToast: (id: string) => void
  clearAllToasts: () => void
}

const NotificationContext = createContext<NotificationContextValue | undefined>(
  undefined
)

interface NotificationProviderProps {
  children: React.ReactNode
}

export function NotificationProvider({ children }: NotificationProviderProps) {
  const [toasts, setToasts] = useState<Toast[]>([])

  const showToast = useCallback(
    (message: string, type: ToastType = 'info', duration: number = 5000) => {
      const id = Date.now().toString() + Math.random().toString(36).substr(2, 9)

      const toast: Toast = {
        id,
        message,
        type,
        duration,
      }

      setToasts((prev) => [...prev, toast])

      // Auto-remove toast after duration
      if (duration > 0) {
        setTimeout(() => {
          removeToast(id)
        }, duration)
      }
    },
    []
  )

  const removeToast = useCallback((id: string) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id))
  }, [])

  const clearAllToasts = useCallback(() => {
    setToasts([])
  }, [])

  const value: NotificationContextValue = {
    toasts,
    showToast,
    removeToast,
    clearAllToasts,
  }

  return (
    <NotificationContext.Provider value={value}>
      {children}
    </NotificationContext.Provider>
  )
}

export function useNotification(): NotificationContextValue {
  const context = useContext(NotificationContext)

  if (context === undefined) {
    throw new Error(
      'useNotification must be used within a NotificationProvider'
    )
  }

  return context
}
