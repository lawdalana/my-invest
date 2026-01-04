/**
 * Watchlists Page
 * Manage custom watchlists
 */

import { useState } from 'react'
import { Plus, Star, Trash2, Edit } from 'lucide-react'
import { Navigation, Button, Card, Badge, Input } from '@/components'
import { Watchlist } from '@/types'
import { useNotification } from '@/contexts'

// Mock data
const MOCK_WATCHLISTS: Watchlist[] = [
  {
    id: '1',
    userId: '1',
    name: 'Tech Stocks',
    description: 'My favorite technology companies',
    isDefault: true,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
    assets: [
      {
        symbol: 'AAPL',
        name: 'Apple Inc.',
        type: 'stock',
        addedAt: new Date().toISOString(),
      },
      {
        symbol: 'MSFT',
        name: 'Microsoft Corporation',
        type: 'stock',
        addedAt: new Date().toISOString(),
      },
    ],
  },
  {
    id: '2',
    userId: '1',
    name: 'Crypto Portfolio',
    description: 'Cryptocurrency investments',
    isDefault: false,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
    assets: [
      {
        symbol: 'BTC',
        name: 'Bitcoin',
        type: 'crypto',
        addedAt: new Date().toISOString(),
      },
    ],
  },
]

export function WatchlistsPage() {
  const { showToast } = useNotification()
  const [watchlists, setWatchlists] = useState<Watchlist[]>(MOCK_WATCHLISTS)
  const [isCreating, setIsCreating] = useState(false)
  const [newWatchlistName, setNewWatchlistName] = useState('')

  const handleCreateWatchlist = () => {
    if (!newWatchlistName.trim()) {
      showToast('Please enter a watchlist name', 'error')
      return
    }

    const newWatchlist: Watchlist = {
      id: Date.now().toString(),
      userId: '1',
      name: newWatchlistName,
      isDefault: false,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      assets: [],
    }

    setWatchlists([...watchlists, newWatchlist])
    setNewWatchlistName('')
    setIsCreating(false)
    showToast('Watchlist created successfully', 'success')
  }

  const handleDeleteWatchlist = (id: string) => {
    setWatchlists(watchlists.filter((w) => w.id !== id))
    showToast('Watchlist deleted', 'success')
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
                />
                <Button variant="primary" onClick={handleCreateWatchlist}>
                  Create
                </Button>
                <Button
                  variant="ghost"
                  onClick={() => {
                    setIsCreating(false)
                    setNewWatchlistName('')
                  }}
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
                    {watchlist.description && (
                      <p className="text-sm text-text-secondary">
                        {watchlist.description}
                      </p>
                    )}
                  </div>

                  <div className="flex gap-2">
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => showToast('Edit coming soon', 'info')}
                    >
                      <Edit className="w-4 h-4" />
                    </Button>
                    {!watchlist.isDefault && (
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleDeleteWatchlist(watchlist.id)}
                      >
                        <Trash2 className="w-4 h-4 text-danger" />
                      </Button>
                    )}
                  </div>
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
                          onClick={() => showToast('Remove coming soon', 'info')}
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
                    <Button
                      variant="ghost"
                      size="sm"
                      className="mt-2"
                      onClick={() => showToast('Add asset coming soon', 'info')}
                    >
                      Add Asset
                    </Button>
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
