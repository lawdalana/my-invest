/**
 * Global Setup for Playwright Integration Tests
 * Runs once before all tests
 */

import { connectToDatabase } from './db.helper'

async function globalSetup() {
  console.log('\n🚀 Starting integration test setup...')

  // Connect to MongoDB
  await connectToDatabase()

  console.log('✅ Global setup complete\n')
}

export default globalSetup
