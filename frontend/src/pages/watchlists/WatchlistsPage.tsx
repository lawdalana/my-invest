/**
 * Watchlists Page
 * Manage custom watchlists
 */

import { useState } from 'react'
import { Plus, Star, Trash2, Edit, Check, X, Loader2 } from 'lucide-react'
import { Navigation, Button, Card, Badge, Input, Skeleton } from '@/components'
import { useNotification } from '@/contexts'
import { useWatchlists } from '@/hooks'

export function WatchlistsPage() {
  const { showToast } = useNotification()
  const {
    watchlists,
    isLoading,
    error,
    createWatchlist,
    updateWatchlist,
    deleteWatchlist,
    removeAsset,
  } = useWatchlists()

  const [isCreating, setIsCreating] = useState(false)
  const [newWatchlistName, setNewWatchlistName] = useState('')
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [editingId, setEditingId] = useState<string | null>(null)
  const [editingName, setEditingName] = useState('')

  const handleCreateWatchlist = async () => {
    if (!newWatchlistName.trim()) {
      showToast('Please enter a watchlist name', 'error')
      return
    }

    setIsSubmitting(true)
    const result = await createWatchlist(newWatchlistName)
    setIsSubmitting(false)

    if (result) {
      setNewWatchlistName('')
      setIsCreating(false)
      showToast('Watchlist created successfully', 'success')
    } else {
      showToast('Failed to create watchlist', 'error')
    }
  }

  const handleDeleteWatchlist = async (id: string) => {
    const success = await deleteWatchlist(id)
    if (success) {
      showToast('Watchlist deleted', 'success')
    } else {
      showToast('Failed to delete watchlist', 'error')
    }
  }

  const handleStartEdit = (id: string, name: string) => {
    setEditingId(id)
    setEditingName(name)
  }

  const handleSaveEdit = async () => {
    if (!editingId || !editingName.trim()) return

    setIsSubmitting(true)
    const result = await updateWatchlist(editingId, editingName)
    setIsSubmitting(false)

    if (result) {
      showToast('Watchlist updated', 'success')
      setEditingId(null)
      setEditingName('')
    } else {
      showToast('Failed to update watchlist', 'error')
    }
  }

  const handleCancelEdit = () => {
    setEditingId(null)
    setEditingName('')
  }

  const handleRemoveAsset = async (watchlistId: string, symbol: string) => {
    const result = await removeAsset(watchlistId, symbol)
    if (result) {
      showToast('Asset removed', 'success')
    } else {
      showToast('Failed to remove asset', 'error')
    }
  }

  if (isLoading) {
    return (
      <div className="min-h-screen bg-background-primary">
        <Navigation />
        <main className="pt-20 pb-8">
          <div className="container-custom max-w-4xl">
            <div className="mb-6">
              <Skeleton variant="title" className="w-48 mb-2" />
              <Skeleton variant="text" className="w-64" />
            </div>
            <div className="space-y-4">
              {[1, 2, 3].map((i) => (
                <Card key={i}>
                  <Skeleton variant="title" className="w-32 mb-4" />
                  <div className="space-y-2">
                    <Skeleton variant="rect" className="h-14" />
                    <Skeleton variant="rect" className="h-14" />
                  </div>
                </Card>
              ))}
            </div>
          </div>
        </main>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-background-primary">
      <Navigation />

      <main className="pt-20 pb-8">
        <div className="container-custom max-w-4xl">
          {/* Header */}
          <div className="flex items-center justify-between mb-6">
            <div>
              <h1 className="text-3xl font-bold text-text-primary mb-2">
                Watchlists
              </h1>
              <p className="text-text-secondary">
                Organize your favorite assets
              </p>
            </div>

            <Button
              variant="primary"
              onClick={() => setIsCreating(true)}
            >
              <Plus className="w-4 h-4" />
              New Watchlist
            </Button>
          </div>

          {/* Error message */}
          {error && (
            <div className="mb-6 p-4 bg-danger/10 border border-danger rounded-lg">
              <p className="text-danger">{error}</p>
            </div>
          )}

          {/* Create Watchlist Form */}
          {isCreating && (
            <Card className="mb-6">
              <h3 className="text-lg font-semibold text-text-primary mb-4">
                Create New Watchlist
              </h3>

              <div className="flex gap-3">
                <Input
                  placeholder="Watchlist name"
                  value={newWatchlistName}
                  onChange={(e) => setNewWatchlistName(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') handleCreateWatchlist()
                    if (e.key === 'Escape') {
                      setIsCreating(false)
                      setNewWatchlistName('')
                    }
                  }}
                  autoFocus
                  disabled={isSubmitting}
                />
                <Button
                  variant="primary"
                  onClick={handleCreateWatchlist}
                  disabled={isSubmitting}
                >
                  {isSubmitting ? (
                    <Loader2 className="w-4 h-4 animate-spin" />
                  ) : (
                    'Create'
                  )}
                </Button>
                <Button
                  variant="ghost"
                  onClick={() => {
                    setIsCreating(false)
                    setNewWatchlistName('')
                  }}
                  disabled={isSubmitting}
                >
                  Cancel
                </Button>
              </div>
            </Card>
          )}

          {/* Watchlists */}
          <div className="space-y-4">
            {watchlists.map((watchlist) => (
              <Card key={watchlist.id}>
                <div className="flex items-start justify-between mb-4">
                  <div className="flex-1">
                    {editingId === watchlist.id ? (
                      <div className="flex items-center gap-2">
                        <Input
                          value={editingName}
                          onChange={(e) => setEditingName(e.target.value)}
                          onKeyDown={(e) => {
                            if (e.key === 'Enter') handleSaveEdit()
                            if (e.key === 'Escape') handleCancelEdit()
                          }}
                          autoFocus
                          disabled={isSubmitting}
                          className="max-w-xs"
                        />
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={handleSaveEdit}
                          disabled={isSubmitting}
                        >
                          {isSubmitting ? (
                            <Loader2 className="w-4 h-4 animate-spin" />
                          ) : (
                            <Check className="w-4 h-4 text-success" />
                          )}
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={handleCancelEdit}
                          disabled={isSubmitting}
                        >
                          <X className="w-4 h-4" />
                        </Button>
                      </div>
                    ) : (
                      <div className="flex items-center gap-2 mb-1">
                        <h3 className="text-lg font-semibold text-text-primary">
                          {watchlist.name}
                        </h3>
                        {watchlist.isDefault && (
                          <Badge variant="info" size="sm">
                            Default
                          </Badge>
                        )}
                      </div>
                    )}
                    {watchlist.description && !editingId && (
                      <p className="text-sm text-text-secondary">
                        {watchlist.description}
                      </p>
                    )}
                  </div>

                  {editingId !== watchlist.id && (
                    <div className="flex gap-2">
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleStartEdit(watchlist.id, watchlist.name)}
                      >
                        <Edit className="w-4 h-4" />
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleDeleteWatchlist(watchlist.id)}
                      >
                        <Trash2 className="w-4 h-4 text-danger" />
                      </Button>
                    </div>
                  )}
                </div>

                {/* Assets in Watchlist */}
                {watchlist.assets.length > 0 ? (
                  <div className="space-y-2">
                    {watchlist.assets.map((asset) => (
                      <div
                        key={asset.symbol}
                        className="flex items-center justify-between p-3 bg-background-tertiary rounded"
                      >
                        <div className="flex items-center gap-3">
                          <div>
                            <p className="font-medium text-text-primary">
                              {asset.symbol}
                            </p>
                            <p className="text-sm text-text-secondary">
                              {asset.name}
                            </p>
                          </div>
                        </div>

                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => handleRemoveAsset(watchlist.id, asset.symbol)}
                        >
                          <Trash2 className="w-4 h-4" />
                        </Button>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-center py-8 border-2 border-dashed border-border-primary rounded">
                    <Star className="w-8 h-8 text-text-tertiary mx-auto mb-2" />
                    <p className="text-text-secondary">
                      No assets in this watchlist yet
                    </p>
                    <p className="text-text-tertiary text-sm mt-1">
                      Search for assets and add them from the asset detail page
                    </p>
                  </div>
                )}
              </Card>
            ))}
          </div>

          {watchlists.length === 0 && !isCreating && (
            <div className="text-center py-12">
              <Star className="w-16 h-16 text-text-tertiary mx-auto mb-4" />
              <p className="text-text-secondary text-lg mb-4">
                No watchlists yet
              </p>
              <Button variant="primary" onClick={() => setIsCreating(true)}>
                <Plus className="w-4 h-4" />
                Create Your First Watchlist
              </Button>
            </div>
          )}
        </div>
      </main>
    </div>
  )
}
