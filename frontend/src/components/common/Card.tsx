/**
 * Card Component
 * Container card with optional hover effect
 */

import React from 'react'

export interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  hoverable?: boolean
  children: React.ReactNode
}

export function Card({
  hoverable = false,
  children,
  className = '',
  ...props
}: CardProps) {
  const baseStyles =
    'bg-background-secondary border border-border-primary rounded p-4 shadow-card'

  const hoverStyles = hoverable
    ? 'cursor-pointer transition-all duration-200 hover:shadow-card-hover hover:border-border-hover'
    : ''

  const combinedClassName = `${baseStyles} ${hoverStyles} ${className}`

  return (
    <div className={combinedClassName} {...props}>
      {children}
    </div>
  )
}
