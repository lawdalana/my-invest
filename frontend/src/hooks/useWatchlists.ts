/**
 * useWatchlists Hook
 * Manages watchlist state with CRUD operations
 */

import { useState, useEffect, useCallback } from 'react'
import type { Watchlist } from '@/types'
import * as watchlistService from '@/services/watchlist.service'
import { getErrorMessage } from '@/services/api/client'

interface UseWatchlistsResult {
  watchlists: Watchlist[]
  isLoading: boolean
  error: string | null
  createWatchlist: (name: string) => Promise<Watchlist | null>
  updateWatchlist: (id: string, name: string) => Promise<Watchlist | null>
  deleteWatchlist: (id: string) => Promise<boolean>
  addAsset: (watchlistId: string, symbol: string) => Promise<Watchlist | null>
  removeAsset: (watchlistId: string, symbol: string) => Promise<Watchlist | null>
  refetch: () => Promise<void>
}

export function useWatchlists(): UseWatchlistsResult {
  const [watchlists, setWatchlists] = useState<Watchlist[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchWatchlists = useCallback(async () => {
    setIsLoading(true)
    setError(null)

    try {
      const data = await watchlistService.getAllWatchlists()
      setWatchlists(data)
    } catch (err) {
      const errorMessage = getErrorMessage(err)
      setError(errorMessage)
      console.error('Failed to fetch watchlists:', errorMessage)
    } finally {
      setIsLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchWatchlists()
  }, [fetchWatchlists])

  const createWatchlist = useCallback(async (name: string): Promise<Watchlist | null> => {
    try {
      const newWatchlist = await watchlistService.createWatchlist(name)
      setWatchlists((prev) => [...prev, newWatchlist])
      return newWatchlist
    } catch (err) {
      const errorMessage = getErrorMessage(err)
      setError(errorMessage)
      console.error('Failed to create watchlist:', errorMessage)
      return null
    }
  }, [])

  const updateWatchlist = useCallback(
    async (id: string, name: string): Promise<Watchlist | null> => {
      try {
        const updatedWatchlist = await watchlistService.updateWatchlist(id, name)
        setWatchlists((prev) =>
          prev.map((w) => (w.id === id ? updatedWatchlist : w))
        )
        return updatedWatchlist
      } catch (err) {
        const errorMessage = getErrorMessage(err)
        setError(errorMessage)
        console.error('Failed to update watchlist:', errorMessage)
        return null
      }
    },
    []
  )

  const deleteWatchlist = useCallback(async (id: string): Promise<boolean> => {
    try {
      await watchlistService.deleteWatchlist(id)
      setWatchlists((prev) => prev.filter((w) => w.id !== id))
      return true
    } catch (err) {
      const errorMessage = getErrorMessage(err)
      setError(errorMessage)
      console.error('Failed to delete watchlist:', errorMessage)
      return false
    }
  }, [])

  const addAsset = useCallback(
    async (watchlistId: string, symbol: string): Promise<Watchlist | null> => {
      try {
        const updatedWatchlist = await watchlistService.addAssetToWatchlist(
          watchlistId,
          symbol
        )
        setWatchlists((prev) =>
          prev.map((w) => (w.id === watchlistId ? updatedWatchlist : w))
        )
        return updatedWatchlist
      } catch (err) {
        const errorMessage = getErrorMessage(err)
        setError(errorMessage)
        console.error('Failed to add asset to watchlist:', errorMessage)
        return null
      }
    },
    []
  )

  const removeAsset = useCallback(
    async (watchlistId: string, symbol: string): Promise<Watchlist | null> => {
      try {
        const updatedWatchlist = await watchlistService.removeAssetFromWatchlist(
          watchlistId,
          symbol
        )
        setWatchlists((prev) =>
          prev.map((w) => (w.id === watchlistId ? updatedWatchlist : w))
        )
        return updatedWatchlist
      } catch (err) {
        const errorMessage = getErrorMessage(err)
        setError(errorMessage)
        console.error('Failed to remove asset from watchlist:', errorMessage)
        return null
      }
    },
    []
  )

  return {
    watchlists,
    isLoading,
    error,
    createWatchlist,
    updateWatchlist,
    deleteWatchlist,
    addAsset,
    removeAsset,
    refetch: fetchWatchlists,
  }
}
