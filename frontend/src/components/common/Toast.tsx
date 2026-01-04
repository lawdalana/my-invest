/**
 * Toast Component
 * Notification toast with auto-dismiss
 */

import React, { useEffect } from 'react'
import { X, CheckCircle, XCircle, AlertTriangle, Info } from 'lucide-react'
import { ToastType } from '@/contexts'

export interface ToastProps {
  id: string
  message: string
  type: ToastType
  onClose: (id: string) => void
  duration?: number
}

const typeConfig: Record<
  ToastType,
  {
    icon: React.ElementType
    borderColor: string
    bgColor: string
    iconColor: string
  }
> = {
  success: {
    icon: CheckCircle,
    borderColor: 'border-l-success',
    bgColor: 'bg-success-light',
    iconColor: 'text-success',
  },
  error: {
    icon: XCircle,
    borderColor: 'border-l-danger',
    bgColor: 'bg-danger-light',
    iconColor: 'text-danger',
  },
  warning: {
    icon: AlertTriangle,
    borderColor: 'border-l-warning',
    bgColor: 'bg-warning-light',
    iconColor: 'text-warning',
  },
  info: {
    icon: Info,
    borderColor: 'border-l-info',
    bgColor: 'bg-info-light',
    iconColor: 'text-info',
  },
}

export function Toast({ id, message, type, onClose, duration = 5000 }: ToastProps) {
  const config = typeConfig[type]
  const Icon = config.icon

  useEffect(() => {
    if (duration <= 0) return

    const timer = setTimeout(() => {
      onClose(id)
    }, duration)

    return () => clearTimeout(timer)
  }, [id, duration, onClose])

  return (
    <div
      className={`
        flex items-start gap-3 p-4 min-w-[300px] max-w-md
        bg-background-secondary border border-border-primary ${config.borderColor} border-l-4
        rounded shadow-dropdown
        animate-toast-slide-in
      `}
      role="alert"
    >
      <Icon className={`w-5 h-5 flex-shrink-0 ${config.iconColor}`} />

      <p className="flex-1 text-sm text-text-primary">{message}</p>

      <button
        onClick={() => onClose(id)}
        className="flex-shrink-0 text-text-tertiary hover:text-text-primary transition-colors"
        aria-label="Close notification"
      >
        <X className="w-4 h-4" />
      </button>
    </div>
  )
}

/**
 * Toast Container Component
 * Renders all active toasts
 */
export interface ToastContainerProps {
  toasts: Array<{
    id: string
    message: string
    type: ToastType
    duration?: number
  }>
  onClose: (id: string) => void
}

export function ToastContainer({ toasts, onClose }: ToastContainerProps) {
  if (toasts.length === 0) return null

  return (
    <div className="toast-container">
      {toasts.map((toast) => (
        <Toast key={toast.id} {...toast} onClose={onClose} />
      ))}
    </div>
  )
}
