/**
 * Skeleton Component
 * Loading placeholder with animated pulse effect
 */

import React from 'react'

export type SkeletonVariant = 'text' | 'title' | 'circle' | 'card' | 'rect'

export interface SkeletonProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: SkeletonVariant
  width?: string | number
  height?: string | number
}

const variantStyles: Record<SkeletonVariant, string> = {
  text: 'h-4 rounded',
  title: 'h-6 rounded',
  circle: 'rounded-full',
  card: 'h-32 rounded',
  rect: 'rounded',
}

export function Skeleton({
  variant = 'text',
  width,
  height,
  className = '',
  style,
  ...props
}: SkeletonProps) {
  const baseStyles = 'skeleton'

  const widthStyle = width
    ? typeof width === 'number'
      ? `${width}px`
      : width
    : variant === 'circle'
    ? '40px'
    : '100%'

  const heightStyle = height
    ? typeof height === 'number'
      ? `${height}px`
      : height
    : variant === 'circle'
    ? '40px'
    : undefined

  const combinedClassName = `${baseStyles} ${variantStyles[variant]} ${className}`

  const combinedStyle = {
    width: widthStyle,
    height: heightStyle,
    ...style,
  }

  return <div className={combinedClassName} style={combinedStyle} {...props} />
}

/**
 * Skeleton Group Components for common loading patterns
 */
export function SkeletonCard() {
  return (
    <div className="bg-background-secondary border border-border-primary rounded p-4">
      <Skeleton variant="title" className="mb-2" width="60%" />
      <Skeleton variant="text" className="mb-2" width="40%" />
      <Skeleton variant="text" className="mb-4" width="80%" />
      <div className="flex gap-2">
        <Skeleton variant="rect" width={80} height={32} />
        <Skeleton variant="rect" width={80} height={32} />
      </div>
    </div>
  )
}

export function SkeletonAssetCard() {
  return (
    <div className="bg-background-secondary border border-border-primary rounded p-4">
      <div className="flex items-start justify-between mb-3">
        <div className="flex-1">
          <Skeleton variant="title" className="mb-1" width="40%" />
          <Skeleton variant="text" width="60%" />
        </div>
        <Skeleton variant="rect" width={50} height={20} />
      </div>
      <div className="space-y-2">
        <Skeleton variant="text" width="30%" />
        <Skeleton variant="text" width="50%" />
        <Skeleton variant="text" width="40%" />
      </div>
    </div>
  )
}

export function SkeletonChart() {
  return (
    <div className="bg-background-secondary border border-border-primary rounded p-4">
      <div className="flex items-center justify-between mb-4">
        <div className="flex-1">
          <Skeleton variant="title" className="mb-1" width="30%" />
          <Skeleton variant="text" width="20%" />
        </div>
        <div className="flex gap-2">
          {[1, 2, 3, 4, 5].map((i) => (
            <Skeleton key={i} variant="rect" width={40} height={32} />
          ))}
        </div>
      </div>
      <Skeleton variant="rect" height={400} />
    </div>
  )
}
