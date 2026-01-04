/**
 * Search Input Component
 * Autocomplete search with debounced input and keyboard navigation
 */

import React, { useState, useRef, useEffect } from 'react'
import { Search, X, Loader2 } from 'lucide-react'
import { AssetSearchResult } from '@/types'
import { debounce, DELAYS } from '@/utils'
import { Input } from '@/components/common'

export interface SearchInputProps {
  placeholder?: string
  onSearch: (query: string) => Promise<AssetSearchResult[]>
  onSelect: (result: AssetSearchResult) => void
  className?: string
}

export function SearchInput({
  placeholder = 'Search assets...',
  onSearch,
  onSelect,
  className = '',
}: SearchInputProps) {
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<AssetSearchResult[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [isOpen, setIsOpen] = useState(false)
  const [selectedIndex, setSelectedIndex] = useState(-1)

  const inputRef = useRef<HTMLInputElement>(null)
  const dropdownRef = useRef<HTMLDivElement>(null)

  // Debounced search function
  const debouncedSearch = useRef(
    debounce(async (searchQuery: string) => {
      if (searchQuery.trim().length < 2) {
        setResults([])
        setIsLoading(false)
        return
      }

      try {
        const searchResults = await onSearch(searchQuery)
        setResults(searchResults)
        setIsOpen(searchResults.length > 0)
      } catch (error) {
        console.error('Search error:', error)
        setResults([])
      } finally {
        setIsLoading(false)
      }
    }, DELAYS.SEARCH_DEBOUNCE)
  ).current

  // Handle input change
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value
    setQuery(value)
    setSelectedIndex(-1)

    if (value.trim().length >= 2) {
      setIsLoading(true)
      debouncedSearch(value)
    } else {
      setResults([])
      setIsOpen(false)
      setIsLoading(false)
    }
  }

  // Handle result selection
  const handleSelect = (result: AssetSearchResult) => {
    onSelect(result)
    setQuery('')
    setResults([])
    setIsOpen(false)
    setSelectedIndex(-1)
    inputRef.current?.blur()
  }

  // Handle clear button
  const handleClear = () => {
    setQuery('')
    setResults([])
    setIsOpen(false)
    setSelectedIndex(-1)
    inputRef.current?.focus()
  }

  // Keyboard navigation
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (!isOpen || results.length === 0) return

    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault()
        setSelectedIndex((prev) =>
          prev < results.length - 1 ? prev + 1 : prev
        )
        break

      case 'ArrowUp':
        e.preventDefault()
        setSelectedIndex((prev) => (prev > 0 ? prev - 1 : -1))
        break

      case 'Enter':
        e.preventDefault()
        if (selectedIndex >= 0 && results[selectedIndex]) {
          handleSelect(results[selectedIndex])
        }
        break

      case 'Escape':
        e.preventDefault()
        setIsOpen(false)
        setSelectedIndex(-1)
        break
    }
  }

  // Click outside to close
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node) &&
        !inputRef.current?.contains(event.target as Node)
      ) {
        setIsOpen(false)
        setSelectedIndex(-1)
      }
    }

    document.addEventListener('mousedown', handleClickOutside)
    return () => document.removeEventListener('mousedown', handleClickOutside)
  }, [])

  return (
    <div className={`relative ${className}`}>
      <div className="relative">
        <Input
          ref={inputRef}
          type="text"
          value={query}
          onChange={handleChange}
          onKeyDown={handleKeyDown}
          placeholder={placeholder}
          icon={isLoading ? Loader2 : query ? X : Search}
          onIconClick={query ? handleClear : undefined}
          className={isLoading ? 'pr-10' : ''}
        />
        {isLoading && (
          <div className="absolute right-3 top-1/2 -translate-y-1/2">
            <Loader2 className="w-4 h-4 animate-spin text-text-tertiary" />
          </div>
        )}
      </div>

      {/* Results Dropdown */}
      {isOpen && results.length > 0 && (
        <div
          ref={dropdownRef}
          className="absolute top-full left-0 right-0 mt-2 dropdown-menu max-h-80 overflow-y-auto z-50"
        >
          {results.map((result, index) => (
            <button
              key={`${result.symbol}-${result.type}`}
              type="button"
              onClick={() => handleSelect(result)}
              className={`
                w-full px-4 py-3 text-left
                transition-colors duration-150
                ${
                  index === selectedIndex
                    ? 'bg-background-tertiary'
                    : 'hover:bg-background-tertiary'
                }
                ${index !== results.length - 1 ? 'border-b border-border-primary' : ''}
              `}
            >
              <div className="flex items-center justify-between">
                <div className="flex-1 min-w-0">
                  <p className="font-medium text-text-primary truncate">
                    {result.symbol}
                  </p>
                  <p className="text-sm text-text-secondary truncate">
                    {result.name}
                  </p>
                </div>
                <span className="ml-2 px-2 py-1 text-xs text-text-tertiary bg-background-tertiary rounded">
                  {result.type.toUpperCase()}
                </span>
              </div>
            </button>
          ))}
        </div>
      )}
    </div>
  )
}
