/**
 * Debounce and Throttle Utilities
 * Performance optimization helpers for event handlers
 */

/**
 * Debounce function - delays execution until after wait time has elapsed since last call
 * Use for: search input, form validation, window resize
 * @param func - Function to debounce
 * @param wait - Wait time in milliseconds
 * @returns Debounced function
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout> | null = null

  return function debouncedFunction(...args: Parameters<T>) {
    if (timeoutId !== null) {
      clearTimeout(timeoutId)
    }

    timeoutId = setTimeout(() => {
      func(...args)
    }, wait)
  }
}

/**
 * Throttle function - ensures function is called at most once per specified time period
 * Use for: scroll events, price updates, window resize
 * @param func - Function to throttle
 * @param limit - Minimum time between calls in milliseconds
 * @returns Throttled function
 */
export function throttle<T extends (...args: unknown[]) => unknown>(
  func: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle: boolean = false

  return function throttledFunction(...args: Parameters<T>) {
    if (!inThrottle) {
      func(...args)
      inThrottle = true
      setTimeout(() => {
        inThrottle = false
      }, limit)
    }
  }
}

/**
 * Create a debounced version of a promise-returning function
 * Cancels previous pending promises when called again
 * @param func - Async function to debounce
 * @param wait - Wait time in milliseconds
 * @returns Debounced async function
 */
export function debounceAsync<T extends (...args: unknown[]) => Promise<unknown>>(
  func: T,
  wait: number
): (...args: Parameters<T>) => Promise<ReturnType<T>> {
  let timeoutId: ReturnType<typeof setTimeout> | null = null
  let pendingPromise: Promise<ReturnType<T>> | null = null

  return function debouncedAsyncFunction(
    ...args: Parameters<T>
  ): Promise<ReturnType<T>> {
    if (timeoutId !== null) {
      clearTimeout(timeoutId)
    }

    if (pendingPromise) {
      // Cancel previous promise (won't reject, just won't resolve)
      pendingPromise = null
    }

    pendingPromise = new Promise((resolve) => {
      timeoutId = setTimeout(async () => {
        const result = await func(...args)
        resolve(result as ReturnType<T>)
      }, wait)
    })

    return pendingPromise
  }
}

/**
 * Request Animation Frame throttle - ensures function is called at most once per animation frame
 * Use for: smooth animations, frequently updating DOM
 * @param func - Function to throttle
 * @returns RAF throttled function
 */
export function rafThrottle<T extends (...args: unknown[]) => unknown>(
  func: T
): (...args: Parameters<T>) => void {
  let rafId: number | null = null

  return function rafThrottledFunction(...args: Parameters<T>) {
    if (rafId !== null) {
      return
    }

    rafId = requestAnimationFrame(() => {
      func(...args)
      rafId = null
    })
  }
}

/**
 * Leading edge debounce - calls function immediately on first call, then debounces subsequent calls
 * Use for: button clicks with debounce protection
 * @param func - Function to debounce
 * @param wait - Wait time in milliseconds
 * @returns Leading edge debounced function
 */
export function debounceLeading<T extends (...args: unknown[]) => unknown>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout> | null = null
  let lastCallTime = 0

  return function debouncedLeadingFunction(...args: Parameters<T>) {
    const now = Date.now()

    if (timeoutId !== null) {
      clearTimeout(timeoutId)
    }

    // Call immediately if enough time has passed
    if (now - lastCallTime >= wait) {
      func(...args)
      lastCallTime = now
    }

    // Set up debounce for trailing edge
    timeoutId = setTimeout(() => {
      func(...args)
      lastCallTime = Date.now()
    }, wait)
  }
}
