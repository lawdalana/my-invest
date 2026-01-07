/**
 * Database Helper for Integration Tests
 * Direct MongoDB access for verifying data after frontend actions
 */

import { MongoClient, Db, Collection, ObjectId } from 'mongodb'

// MongoDB connection configuration
const MONGODB_URL = process.env.MONGODB_URL || 'mongodb://localhost:27017'
const DATABASE_NAME = process.env.DATABASE_NAME || 'myinvest'

let client: MongoClient | null = null
let db: Db | null = null

// Collection interfaces (matching backend schema)
interface UserDocument {
  _id: ObjectId
  email: string
  username: string
  password_hash: string
  created_at: Date
  updated_at: Date
}

interface WatchlistDocument {
  _id: ObjectId
  user_id: ObjectId
  name: string
  description?: string
  is_default: boolean
  assets: Array<{
    symbol: string
    name: string
    added_at: Date
  }>
  created_at: Date
  updated_at: Date
}

interface AssetDocument {
  _id: ObjectId
  symbol: string
  name: string
  asset_type: string
  price: number
  change_24h: number
  change_percent_24h: number
  volume_24h: number
  market_cap: number
  last_updated: Date
}

/**
 * Connect to MongoDB
 */
export async function connectToDatabase(): Promise<Db> {
  if (db) return db

  client = new MongoClient(MONGODB_URL)
  await client.connect()
  db = client.db(DATABASE_NAME)

  console.log(`Connected to MongoDB: ${DATABASE_NAME}`)
  return db
}

/**
 * Disconnect from MongoDB
 */
export async function disconnectFromDatabase(): Promise<void> {
  if (client) {
    await client.close()
    client = null
    db = null
    console.log('Disconnected from MongoDB')
  }
}

/**
 * Get users collection
 */
export function getUsersCollection(): Collection<UserDocument> {
  if (!db) throw new Error('Database not connected')
  return db.collection<UserDocument>('users')
}

/**
 * Get watchlists collection
 */
export function getWatchlistsCollection(): Collection<WatchlistDocument> {
  if (!db) throw new Error('Database not connected')
  return db.collection<WatchlistDocument>('watchlists')
}

/**
 * Get assets collection
 */
export function getAssetsCollection(): Collection<AssetDocument> {
  if (!db) throw new Error('Database not connected')
  return db.collection<AssetDocument>('assets')
}

/**
 * Find user by email
 */
export async function findUserByEmail(email: string): Promise<UserDocument | null> {
  const users = getUsersCollection()
  return users.findOne({ email })
}

/**
 * Find user by username
 */
export async function findUserByUsername(username: string): Promise<UserDocument | null> {
  const users = getUsersCollection()
  return users.findOne({ username })
}

/**
 * Find watchlists by user email
 */
export async function findWatchlistsByUserEmail(email: string): Promise<WatchlistDocument[]> {
  const user = await findUserByEmail(email)
  if (!user) return []

  const watchlists = getWatchlistsCollection()
  return watchlists.find({ user_id: user._id }).toArray()
}

/**
 * Find watchlist by name for user
 */
export async function findWatchlistByName(
  email: string,
  watchlistName: string
): Promise<WatchlistDocument | null> {
  const user = await findUserByEmail(email)
  if (!user) return null

  const watchlists = getWatchlistsCollection()
  return watchlists.findOne({ user_id: user._id, name: watchlistName })
}

/**
 * Delete user by email (cleanup for tests)
 */
export async function deleteUserByEmail(email: string): Promise<void> {
  const user = await findUserByEmail(email)
  if (user) {
    // Delete user's watchlists first
    const watchlists = getWatchlistsCollection()
    await watchlists.deleteMany({ user_id: user._id })

    // Delete user
    const users = getUsersCollection()
    await users.deleteOne({ _id: user._id })
  }
}

/**
 * Delete watchlist by ID
 */
export async function deleteWatchlistById(id: string): Promise<void> {
  const watchlists = getWatchlistsCollection()
  await watchlists.deleteOne({ _id: new ObjectId(id) })
}

/**
 * Count watchlists for user
 */
export async function countWatchlistsForUser(email: string): Promise<number> {
  const user = await findUserByEmail(email)
  if (!user) return 0

  const watchlists = getWatchlistsCollection()
  return watchlists.countDocuments({ user_id: user._id })
}

/**
 * Check if asset exists in watchlist
 */
export async function isAssetInWatchlist(
  watchlistId: string,
  symbol: string
): Promise<boolean> {
  const watchlists = getWatchlistsCollection()
  const watchlist = await watchlists.findOne({
    _id: new ObjectId(watchlistId),
    'assets.symbol': symbol,
  })
  return watchlist !== null
}

/**
 * Get database instance
 */
export function getDatabase(): Db {
  if (!db) throw new Error('Database not connected')
  return db
}
