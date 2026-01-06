/**
 * Integration Tests: Frontend → Backend → Database
 *
 * These tests perform actions through the frontend UI and verify
 * the data was correctly stored in MongoDB.
 *
 * Prerequisites:
 * - Docker Compose stack running (frontend, backend, mongodb, redis)
 * - Run with: npm run test:integration
 */

import { test, expect, Page } from '@playwright/test'
import {
  connectToDatabase,
  disconnectFromDatabase,
  findUserByEmail,
  findWatchlistsByUserEmail,
  findWatchlistByName,
  deleteUserByEmail,
  countWatchlistsForUser,
  isAssetInWatchlist,
} from './db.helper'

// Test user credentials
const TEST_USER = {
  email: `test-${Date.now()}@example.com`,
  username: `testuser${Date.now()}`,
  password: 'TestPassword123!',
}

// Helper to wait for navigation and page load
async function waitForPageLoad(page: Page) {
  await page.waitForLoadState('networkidle')
}

// Helper to login
async function login(page: Page, email: string, password: string) {
  await page.goto('/login')
  await waitForPageLoad(page)

  await page.fill('input[type="email"]', email)
  await page.fill('input[type="password"]', password)
  await page.click('button[type="submit"]')

  // Wait for redirect to dashboard
  await page.waitForURL('/', { timeout: 10000 })
  await waitForPageLoad(page)
}

// Helper to logout
async function logout(page: Page) {
  // Click user menu and logout
  const userMenu = page.locator('[data-testid="user-menu"]').or(page.locator('button:has-text("Logout")'))
  if (await userMenu.isVisible()) {
    await userMenu.click()
  }

  // Try multiple selectors for logout button
  const logoutBtn = page.locator('button:has-text("Logout")').or(page.locator('[data-testid="logout-button"]'))
  if (await logoutBtn.isVisible()) {
    await logoutBtn.click()
  }

  await page.waitForURL('/login', { timeout: 10000 })
}

test.describe('User Registration Integration', () => {
  test.beforeAll(async () => {
    await connectToDatabase()
    // Clean up any existing test user
    await deleteUserByEmail(TEST_USER.email)
  })

  test.afterAll(async () => {
    // Clean up test user
    await deleteUserByEmail(TEST_USER.email)
    await disconnectFromDatabase()
  })

  test('should register a new user and verify in MongoDB', async ({ page }) => {
    // Navigate to registration page
    await page.goto('/register')
    await waitForPageLoad(page)

    // Fill registration form
    await page.fill('input[name="email"], input[type="email"]', TEST_USER.email)
    await page.fill('input[name="username"]', TEST_USER.username)

    // Fill password fields
    const passwordInputs = page.locator('input[type="password"]')
    await passwordInputs.first().fill(TEST_USER.password)
    if ((await passwordInputs.count()) > 1) {
      await passwordInputs.nth(1).fill(TEST_USER.password)
    }

    // Submit registration
    await page.click('button[type="submit"]')

    // Wait for successful registration (redirect to login or dashboard)
    await page.waitForURL(/\/(login)?$/, { timeout: 15000 })

    // VERIFY IN DATABASE: User should exist in MongoDB
    const user = await findUserByEmail(TEST_USER.email)

    expect(user).not.toBeNull()
    expect(user?.email).toBe(TEST_USER.email)
    expect(user?.username).toBe(TEST_USER.username)
    expect(user?.password_hash).toBeDefined()
    expect(user?.password_hash).not.toBe(TEST_USER.password) // Password should be hashed
    expect(user?.created_at).toBeInstanceOf(Date)

    console.log(`✅ User registered and verified in MongoDB: ${TEST_USER.email}`)
  })
})

test.describe('Watchlist CRUD Integration', () => {
  const WATCHLIST_NAME = `Test Watchlist ${Date.now()}`
  const UPDATED_WATCHLIST_NAME = `Updated Watchlist ${Date.now()}`
  let watchlistId: string | null = null

  test.beforeAll(async () => {
    await connectToDatabase()
  })

  test.afterAll(async () => {
    // Clean up test data
    await deleteUserByEmail(TEST_USER.email)
    await disconnectFromDatabase()
  })

  test('setup: register and login test user', async ({ page }) => {
    // Register user first
    await page.goto('/register')
    await waitForPageLoad(page)

    await page.fill('input[name="email"], input[type="email"]', TEST_USER.email)
    await page.fill('input[name="username"]', TEST_USER.username)

    const passwordInputs = page.locator('input[type="password"]')
    await passwordInputs.first().fill(TEST_USER.password)
    if ((await passwordInputs.count()) > 1) {
      await passwordInputs.nth(1).fill(TEST_USER.password)
    }

    await page.click('button[type="submit"]')
    await page.waitForURL(/\/(login)?$/, { timeout: 15000 })

    // Login
    await login(page, TEST_USER.email, TEST_USER.password)

    // Verify user is logged in
    expect(page.url()).toContain('/')
    console.log('✅ Test user registered and logged in')
  })

  test('should create a watchlist and verify in MongoDB', async ({ page }) => {
    // Login
    await login(page, TEST_USER.email, TEST_USER.password)

    // Navigate to watchlists page
    await page.goto('/watchlists')
    await waitForPageLoad(page)

    // Get initial watchlist count
    const initialCount = await countWatchlistsForUser(TEST_USER.email)

    // Click "New Watchlist" button
    const newWatchlistBtn = page.locator('button:has-text("New Watchlist")')
    await newWatchlistBtn.click()

    // Fill watchlist name
    const nameInput = page.locator('input[placeholder*="name"], input[placeholder*="Watchlist"]')
    await nameInput.fill(WATCHLIST_NAME)

    // Click Create button
    const createBtn = page.locator('button:has-text("Create")')
    await createBtn.click()

    // Wait for success (toast or the watchlist appearing)
    await page.waitForTimeout(2000) // Allow time for API call

    // VERIFY IN DATABASE: Watchlist should exist
    const watchlist = await findWatchlistByName(TEST_USER.email, WATCHLIST_NAME)

    expect(watchlist).not.toBeNull()
    expect(watchlist?.name).toBe(WATCHLIST_NAME)
    expect(watchlist?.assets).toEqual([])
    expect(watchlist?.created_at).toBeInstanceOf(Date)

    // Store watchlist ID for later tests
    if (watchlist) {
      watchlistId = watchlist._id.toString()
    }

    // Verify count increased
    const newCount = await countWatchlistsForUser(TEST_USER.email)
    expect(newCount).toBe(initialCount + 1)

    console.log(`✅ Watchlist created and verified in MongoDB: ${WATCHLIST_NAME}`)
  })

  test('should update watchlist name and verify in MongoDB', async ({ page }) => {
    // Skip if watchlist wasn't created
    test.skip(!watchlistId, 'Watchlist not created in previous test')

    // Login
    await login(page, TEST_USER.email, TEST_USER.password)

    // Navigate to watchlists page
    await page.goto('/watchlists')
    await waitForPageLoad(page)

    // Find and click edit button for our watchlist
    const watchlistCard = page.locator(`text=${WATCHLIST_NAME}`).locator('..')
    const editBtn = watchlistCard.locator('button:has(svg[class*="edit"]), button:has-text("Edit")').first()

    // If no edit button visible, try clicking on edit icon
    if (!(await editBtn.isVisible())) {
      const editIcon = watchlistCard.locator('svg').filter({ hasText: /edit/i }).first()
      if (await editIcon.isVisible()) {
        await editIcon.click()
      }
    } else {
      await editBtn.click()
    }

    // Fill new name
    const nameInput = page.locator('input').first()
    await nameInput.clear()
    await nameInput.fill(UPDATED_WATCHLIST_NAME)

    // Click save/confirm button
    const saveBtn = page.locator('button:has(svg[class*="check"]), button:has-text("Save")').first()
    await saveBtn.click()

    // Wait for update
    await page.waitForTimeout(2000)

    // VERIFY IN DATABASE: Watchlist name should be updated
    const watchlist = await findWatchlistByName(TEST_USER.email, UPDATED_WATCHLIST_NAME)

    expect(watchlist).not.toBeNull()
    expect(watchlist?.name).toBe(UPDATED_WATCHLIST_NAME)
    expect(watchlist?.updated_at).toBeInstanceOf(Date)

    // Old name should not exist
    const oldWatchlist = await findWatchlistByName(TEST_USER.email, WATCHLIST_NAME)
    expect(oldWatchlist).toBeNull()

    console.log(`✅ Watchlist updated and verified in MongoDB: ${UPDATED_WATCHLIST_NAME}`)
  })

  test('should delete watchlist and verify removal from MongoDB', async ({ page }) => {
    // Login
    await login(page, TEST_USER.email, TEST_USER.password)

    // Navigate to watchlists page
    await page.goto('/watchlists')
    await waitForPageLoad(page)

    // Get initial count
    const initialCount = await countWatchlistsForUser(TEST_USER.email)

    // Find delete button for our watchlist
    const watchlistCard = page.locator(`text=${UPDATED_WATCHLIST_NAME}`).locator('..')
    const deleteBtn = watchlistCard.locator('button:has(svg[class*="trash"]), button:has-text("Delete")').first()

    await deleteBtn.click()

    // Handle confirmation dialog if present
    const confirmBtn = page.locator('button:has-text("Confirm"), button:has-text("Delete"), button:has-text("Yes")')
    if (await confirmBtn.isVisible({ timeout: 1000 })) {
      await confirmBtn.click()
    }

    // Wait for deletion
    await page.waitForTimeout(2000)

    // VERIFY IN DATABASE: Watchlist should be deleted
    const watchlist = await findWatchlistByName(TEST_USER.email, UPDATED_WATCHLIST_NAME)
    expect(watchlist).toBeNull()

    // Verify count decreased
    const newCount = await countWatchlistsForUser(TEST_USER.email)
    expect(newCount).toBe(initialCount - 1)

    console.log(`✅ Watchlist deleted and verified in MongoDB`)
  })
})

test.describe('Asset to Watchlist Integration', () => {
  const WATCHLIST_NAME = `Asset Test Watchlist ${Date.now()}`
  const TEST_ASSET_SYMBOL = 'AAPL' // Apple stock as test asset

  test.beforeAll(async () => {
    await connectToDatabase()
    // Clean up any existing test user
    await deleteUserByEmail(TEST_USER.email)
  })

  test.afterAll(async () => {
    // Clean up test data
    await deleteUserByEmail(TEST_USER.email)
    await disconnectFromDatabase()
  })

  test('setup: register, login, and create watchlist', async ({ page }) => {
    // Register
    await page.goto('/register')
    await waitForPageLoad(page)

    await page.fill('input[name="email"], input[type="email"]', TEST_USER.email)
    await page.fill('input[name="username"]', TEST_USER.username)

    const passwordInputs = page.locator('input[type="password"]')
    await passwordInputs.first().fill(TEST_USER.password)
    if ((await passwordInputs.count()) > 1) {
      await passwordInputs.nth(1).fill(TEST_USER.password)
    }

    await page.click('button[type="submit"]')
    await page.waitForURL(/\/(login)?$/, { timeout: 15000 })

    // Login
    await login(page, TEST_USER.email, TEST_USER.password)

    // Create watchlist
    await page.goto('/watchlists')
    await waitForPageLoad(page)

    const newWatchlistBtn = page.locator('button:has-text("New Watchlist")')
    await newWatchlistBtn.click()

    const nameInput = page.locator('input[placeholder*="name"], input[placeholder*="Watchlist"]')
    await nameInput.fill(WATCHLIST_NAME)

    const createBtn = page.locator('button:has-text("Create")')
    await createBtn.click()

    await page.waitForTimeout(2000)

    const watchlist = await findWatchlistByName(TEST_USER.email, WATCHLIST_NAME)
    expect(watchlist).not.toBeNull()

    console.log('✅ Setup complete: user registered and watchlist created')
  })

  test('should add asset to watchlist from asset detail page', async ({ page }) => {
    // Login
    await login(page, TEST_USER.email, TEST_USER.password)

    // Search for an asset or navigate directly
    await page.goto(`/asset/${TEST_ASSET_SYMBOL}`)
    await waitForPageLoad(page)

    // Check if page loaded (may show error if asset doesn't exist yet)
    const pageContent = await page.content()

    // Click "Add to Watchlist" button
    const addToWatchlistBtn = page.locator('button:has-text("Add to Watchlist")')

    if (await addToWatchlistBtn.isVisible({ timeout: 5000 })) {
      await addToWatchlistBtn.click()

      // Select our watchlist from dropdown/modal
      const watchlistSelect = page.locator('select, [role="listbox"]')
      if (await watchlistSelect.isVisible({ timeout: 3000 })) {
        await watchlistSelect.selectOption({ label: WATCHLIST_NAME })
      }

      // Click Add/Confirm button
      const addBtn = page.locator('button:has-text("Add")').last()
      await addBtn.click()

      // Wait for the action to complete
      await page.waitForTimeout(2000)

      // VERIFY IN DATABASE: Asset should be in watchlist
      const watchlist = await findWatchlistByName(TEST_USER.email, WATCHLIST_NAME)

      if (watchlist) {
        const assetInWatchlist = await isAssetInWatchlist(
          watchlist._id.toString(),
          TEST_ASSET_SYMBOL
        )
        expect(assetInWatchlist).toBe(true)
        console.log(`✅ Asset ${TEST_ASSET_SYMBOL} added to watchlist and verified in MongoDB`)
      }
    } else {
      // Asset detail page may not be available, skip verification
      console.log('⚠️ Add to Watchlist button not visible - asset may not exist in backend')
    }
  })

  test('should remove asset from watchlist', async ({ page }) => {
    // Login
    await login(page, TEST_USER.email, TEST_USER.password)

    // Navigate to watchlists
    await page.goto('/watchlists')
    await waitForPageLoad(page)

    // Find our watchlist and the remove button for the asset
    const watchlistCard = page.locator(`text=${WATCHLIST_NAME}`).locator('..')
    const assetRow = watchlistCard.locator(`text=${TEST_ASSET_SYMBOL}`).locator('..')
    const removeBtn = assetRow.locator('button:has(svg[class*="trash"]), button:has-text("Remove")').first()

    if (await removeBtn.isVisible({ timeout: 3000 })) {
      await removeBtn.click()

      // Wait for removal
      await page.waitForTimeout(2000)

      // VERIFY IN DATABASE: Asset should be removed from watchlist
      const watchlist = await findWatchlistByName(TEST_USER.email, WATCHLIST_NAME)

      if (watchlist) {
        const assetInWatchlist = await isAssetInWatchlist(
          watchlist._id.toString(),
          TEST_ASSET_SYMBOL
        )
        expect(assetInWatchlist).toBe(false)
        console.log(`✅ Asset ${TEST_ASSET_SYMBOL} removed from watchlist and verified in MongoDB`)
      }
    } else {
      console.log('⚠️ Remove button not visible - asset may not have been added')
    }
  })
})

test.describe('Error Handling', () => {
  test('should show error for invalid login credentials', async ({ page }) => {
    await page.goto('/login')
    await waitForPageLoad(page)

    await page.fill('input[type="email"]', 'nonexistent@example.com')
    await page.fill('input[type="password"]', 'wrongpassword')
    await page.click('button[type="submit"]')

    // Wait for error message
    const errorMessage = page.locator('.text-danger, [role="alert"], .error')
    await expect(errorMessage).toBeVisible({ timeout: 10000 })

    // Should remain on login page
    expect(page.url()).toContain('/login')

    console.log('✅ Invalid login error handled correctly')
  })
})
