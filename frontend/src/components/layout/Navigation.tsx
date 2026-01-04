/**
 * Navigation Component
 * Top navigation bar with search, theme toggle, and user menu
 */

import React, { useState } from 'react'
import { Link, useNavigate, useLocation } from 'react-router-dom'
import {
  TrendingUp,
  Star,
  Bell,
  Settings,
  Sun,
  Moon,
  Menu,
  X,
  LogOut,
  User,
} from 'lucide-react'
import { useAuth, useTheme } from '@/contexts'
import { Button } from '@/components/common'
import { SearchInput } from '@/components/search/SearchInput'
import { AssetSearchResult } from '@/types'

export interface NavigationProps {
  onSearch?: (query: string) => Promise<AssetSearchResult[]>
}

export function Navigation({ onSearch }: NavigationProps) {
  const { user, logout } = useAuth()
  const { theme, toggleTheme } = useTheme()
  const navigate = useNavigate()
  const location = useLocation()
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false)
  const [isUserMenuOpen, setIsUserMenuOpen] = useState(false)

  const navLinks = [
    { path: '/', label: 'Dashboard', icon: TrendingUp },
    { path: '/watchlists', label: 'Watchlists', icon: Star },
    { path: '/alerts', label: 'Alerts', icon: Bell },
    { path: '/settings', label: 'Settings', icon: Settings },
  ]

  const handleSearch = async (query: string): Promise<AssetSearchResult[]> => {
    if (onSearch) {
      return onSearch(query)
    }
    // Mock search for now
    return []
  }

  const handleSelectAsset = (result: AssetSearchResult) => {
    navigate(`/asset/${result.symbol}`)
    setIsMobileMenuOpen(false)
  }

  const handleLogout = () => {
    logout()
    navigate('/login')
  }

  const isActive = (path: string) => {
    if (path === '/') {
      return location.pathname === '/'
    }
    return location.pathname.startsWith(path)
  }

  return (
    <nav className="fixed top-0 left-0 right-0 z-50 bg-background-secondary border-b border-border-primary">
      <div className="container-custom">
        <div className="flex items-center justify-between h-14">
          {/* Logo */}
          <Link to="/" className="flex items-center gap-2 no-print">
            <TrendingUp className="w-6 h-6 text-accent" />
            <span className="text-lg font-bold text-text-primary hidden sm:inline">
              My-Invest
            </span>
          </Link>

          {/* Desktop Navigation */}
          <div className="hidden md:flex items-center gap-6 flex-1 mx-8">
            {navLinks.slice(0, 3).map((link) => (
              <Link
                key={link.path}
                to={link.path}
                className={`
                  flex items-center gap-2 px-3 py-2 rounded transition-colors
                  ${
                    isActive(link.path)
                      ? 'text-accent bg-accent-light'
                      : 'text-text-secondary hover:text-text-primary hover:bg-background-tertiary'
                  }
                `}
              >
                <link.icon className="w-4 h-4" />
                <span className="text-sm font-medium">{link.label}</span>
              </Link>
            ))}
          </div>

          {/* Search */}
          <div className="hidden md:block flex-1 max-w-md">
            <SearchInput
              placeholder="Search assets..."
              onSearch={handleSearch}
              onSelect={handleSelectAsset}
            />
          </div>

          {/* Right Section */}
          <div className="flex items-center gap-2 ml-4">
            {/* Theme Toggle */}
            <Button
              variant="ghost"
              size="sm"
              onClick={toggleTheme}
              aria-label="Toggle theme"
              className="hidden sm:flex"
            >
              {theme === 'dark' ? (
                <Sun className="w-4 h-4" />
              ) : (
                <Moon className="w-4 h-4" />
              )}
            </Button>

            {/* User Menu */}
            <div className="relative">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => setIsUserMenuOpen(!isUserMenuOpen)}
                aria-label="User menu"
                className="hidden sm:flex"
              >
                <User className="w-4 h-4" />
                <span className="max-w-[100px] truncate">
                  {user?.email?.split('@')[0]}
                </span>
              </Button>

              {isUserMenuOpen && (
                <>
                  <div
                    className="fixed inset-0 z-40"
                    onClick={() => setIsUserMenuOpen(false)}
                  />
                  <div className="absolute right-0 mt-2 w-48 dropdown-menu z-50">
                    <Link
                      to="/settings"
                      className="flex items-center gap-2 px-4 py-2 text-sm text-text-primary hover:bg-background-tertiary transition-colors"
                      onClick={() => setIsUserMenuOpen(false)}
                    >
                      <Settings className="w-4 h-4" />
                      Settings
                    </Link>
                    <button
                      onClick={handleLogout}
                      className="w-full flex items-center gap-2 px-4 py-2 text-sm text-danger hover:bg-background-tertiary transition-colors"
                    >
                      <LogOut className="w-4 h-4" />
                      Logout
                    </button>
                  </div>
                </>
              )}
            </div>

            {/* Mobile Menu Toggle */}
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              aria-label="Toggle menu"
              className="md:hidden"
            >
              {isMobileMenuOpen ? (
                <X className="w-5 h-5" />
              ) : (
                <Menu className="w-5 h-5" />
              )}
            </Button>
          </div>
        </div>

        {/* Mobile Menu */}
        {isMobileMenuOpen && (
          <div className="md:hidden border-t border-border-primary py-4">
            {/* Mobile Search */}
            <div className="mb-4">
              <SearchInput
                placeholder="Search assets..."
                onSearch={handleSearch}
                onSelect={handleSelectAsset}
              />
            </div>

            {/* Mobile Nav Links */}
            <div className="space-y-1">
              {navLinks.map((link) => (
                <Link
                  key={link.path}
                  to={link.path}
                  onClick={() => setIsMobileMenuOpen(false)}
                  className={`
                    flex items-center gap-2 px-3 py-2 rounded transition-colors
                    ${
                      isActive(link.path)
                        ? 'text-accent bg-accent-light'
                        : 'text-text-secondary hover:text-text-primary hover:bg-background-tertiary'
                    }
                  `}
                >
                  <link.icon className="w-4 h-4" />
                  <span className="text-sm font-medium">{link.label}</span>
                </Link>
              ))}

              {/* Mobile Theme Toggle */}
              <button
                onClick={() => {
                  toggleTheme()
                  setIsMobileMenuOpen(false)
                }}
                className="w-full flex items-center gap-2 px-3 py-2 rounded text-text-secondary hover:text-text-primary hover:bg-background-tertiary transition-colors"
              >
                {theme === 'dark' ? (
                  <>
                    <Sun className="w-4 h-4" />
                    <span className="text-sm font-medium">Light Mode</span>
                  </>
                ) : (
                  <>
                    <Moon className="w-4 h-4" />
                    <span className="text-sm font-medium">Dark Mode</span>
                  </>
                )}
              </button>

              {/* Mobile Logout */}
              <button
                onClick={handleLogout}
                className="w-full flex items-center gap-2 px-3 py-2 rounded text-danger hover:bg-background-tertiary transition-colors"
              >
                <LogOut className="w-4 h-4" />
                <span className="text-sm font-medium">Logout</span>
              </button>
            </div>
          </div>
        )}
      </div>
    </nav>
  )
}
