/**
 * Input Component
 * Text input field with label, error state, and optional icon
 */

import { forwardRef } from 'react'
import { LucideIcon } from 'lucide-react'

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string
  error?: string
  icon?: LucideIcon
  onIconClick?: () => void
}

export const Input = forwardRef<HTMLInputElement, InputProps>(function Input({
  label,
  error,
  icon: Icon,
  onIconClick,
  className = '',
  id,
  ...props
}, ref) {
  const inputId = id || label?.toLowerCase().replace(/\s+/g, '-')

  return (
    <div className="w-full">
      {label && (
        <label
          htmlFor={inputId}
          className="block text-sm font-medium text-text-primary mb-1"
        >
          {label}
        </label>
      )}

      <div className="relative">
        <input
          ref={ref}
          id={inputId}
          className={`
            w-full px-3 py-2
            bg-background-secondary
            border ${error ? 'border-danger' : 'border-border-primary'}
            rounded
            text-text-primary placeholder-text-tertiary
            transition-all duration-200
            focus:outline-none focus:ring-2 ${
              error ? 'focus:ring-danger' : 'focus:ring-accent'
            } focus:ring-offset-2 focus:ring-offset-background-primary
            disabled:opacity-50 disabled:cursor-not-allowed
            ${Icon ? 'pr-10' : ''}
            ${className}
          `}
          {...props}
        />

        {Icon && (
          <button
            type="button"
            onClick={onIconClick}
            disabled={!onIconClick}
            className={`
              absolute right-3 top-1/2 -translate-y-1/2
              text-text-tertiary
              ${onIconClick ? 'hover:text-text-primary cursor-pointer' : 'cursor-default'}
              transition-colors duration-200
            `}
            tabIndex={onIconClick ? 0 : -1}
          >
            <Icon className="w-4 h-4" />
          </button>
        )}
      </div>

      {error && <p className="mt-1 text-xs text-danger">{error}</p>}
    </div>
  )
})
