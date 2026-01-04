/**
 * Badge Component
 * Small colored label for status, asset types, etc.
 */

import React from 'react'

export type BadgeVariant = 'success' | 'danger' | 'warning' | 'info' | 'neutral'
export type BadgeSize = 'sm' | 'md'

export interface BadgeProps extends React.HTMLAttributes<HTMLSpanElement> {
  variant?: BadgeVariant
  size?: BadgeSize
  children: React.ReactNode
}

const variantStyles: Record<BadgeVariant, string> = {
  success: 'bg-success-light text-success border-success/20',
  danger: 'bg-danger-light text-danger border-danger/20',
  warning: 'bg-warning-light text-warning border-warning/20',
  info: 'bg-info-light text-info border-info/20',
  neutral: 'bg-background-tertiary text-text-secondary border-border-primary',
}

const sizeStyles: Record<BadgeSize, string> = {
  sm: 'px-1.5 py-0.5 text-2xs',
  md: 'px-2 py-1 text-xs',
}

export function Badge({
  variant = 'neutral',
  size = 'sm',
  children,
  className = '',
  ...props
}: BadgeProps) {
  const baseStyles =
    'inline-flex items-center justify-center font-medium rounded border'

  const combinedClassName = `${baseStyles} ${variantStyles[variant]} ${sizeStyles[size]} ${className}`

  return (
    <span className={combinedClassName} {...props}>
      {children}
    </span>
  )
}
