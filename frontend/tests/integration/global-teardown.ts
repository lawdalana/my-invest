/**
 * Global Teardown for Playwright Integration Tests
 * Runs once after all tests
 */

import { disconnectFromDatabase } from './db.helper'

async function globalTeardown() {
  console.log('\n🧹 Starting integration test teardown...')

  // Disconnect from MongoDB
  await disconnectFromDatabase()

  console.log('✅ Global teardown complete\n')
}

export default globalTeardown
