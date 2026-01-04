/**
 * App Component
 * Main application component with routing and context providers
 */

import React, { Suspense, lazy } from 'react'
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { AuthProvider, ThemeProvider, NotificationProvider } from '@/contexts'
import { ProtectedRoute } from '@/components/auth/ProtectedRoute'
import { ToastContainer } from '@/components'
import { useNotification } from '@/contexts'

// Lazy load pages for better performance
const LoginPage = lazy(() =>
  import('@/pages').then((module) => ({ default: module.LoginPage }))
)
const RegisterPage = lazy(() =>
  import('@/pages').then((module) => ({ default: module.RegisterPage }))
)
const DashboardPage = lazy(() =>
  import('@/pages').then((module) => ({ default: module.DashboardPage }))
)
const AssetDetailPage = lazy(() =>
  import('@/pages').then((module) => ({ default: module.AssetDetailPage }))
)
const WatchlistsPage = lazy(() =>
  import('@/pages').then((module) => ({ default: module.WatchlistsPage }))
)
const SettingsPage = lazy(() =>
  import('@/pages').then((module) => ({ default: module.SettingsPage }))
)

// Loading component
function LoadingFallback() {
  return (
    <div className="min-h-screen bg-background-primary flex items-center justify-center">
      <div className="text-center">
        <div className="spinner w-8 h-8 border-4 border-accent border-t-transparent rounded-full mx-auto mb-4" />
        <p className="text-text-secondary">Loading...</p>
      </div>
    </div>
  )
}

// Toast provider wrapper
function ToastProvider({ children }: { children: React.ReactNode }) {
  const { toasts, removeToast } = useNotification()

  return (
    <>
      {children}
      <ToastContainer toasts={toasts} onClose={removeToast} />
    </>
  )
}

// App Routes
function AppRoutes() {
  return (
    <Suspense fallback={<LoadingFallback />}>
      <Routes>
        {/* Public Routes */}
        <Route path="/login" element={<LoginPage />} />
        <Route path="/register" element={<RegisterPage />} />

        {/* Protected Routes */}
        <Route element={<ProtectedRoute />}>
          <Route path="/" element={<DashboardPage />} />
          <Route path="/asset/:symbol" element={<AssetDetailPage />} />
          <Route path="/watchlists" element={<WatchlistsPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Route>

        {/* Catch-all redirect */}
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </Suspense>
  )
}

function App() {
  return (
    <BrowserRouter>
      <ThemeProvider>
        <AuthProvider>
          <NotificationProvider>
            <ToastProvider>
              <AppRoutes />
            </ToastProvider>
          </NotificationProvider>
        </AuthProvider>
      </ThemeProvider>
    </BrowserRouter>
  )
}

export default App
