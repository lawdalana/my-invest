/**
 * Playwright Configuration
 * Integration tests for frontend → backend → database verification
 */

import { defineConfig, devices } from '@playwright/test'

export default defineConfig({
  testDir: './tests/integration',
  fullyParallel: false, // Run tests sequentially to avoid data conflicts
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1, // Single worker for integration tests
  reporter: 'html',
  timeout: 60000, // 60 seconds per test

  use: {
    baseURL: process.env.FRONTEND_URL || 'http://localhost:3000',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },

  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],

  // Global setup/teardown for database connections
  globalSetup: './tests/integration/global-setup.ts',
  globalTeardown: './tests/integration/global-teardown.ts',
})
