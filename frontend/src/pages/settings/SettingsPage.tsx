/**
 * Settings Page
 * User preferences and settings
 */

import React from 'react'
import { LogOut, User, Moon, Sun, Bell } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { Navigation, Button, Card } from '@/components'
import { useAuth, useTheme, useNotification } from '@/contexts'

export function SettingsPage() {
  const { user, logout } = useAuth()
  const { theme, toggleTheme } = useTheme()
  const { showToast } = useNotification()
  const navigate = useNavigate()

  const handleLogout = () => {
    logout()
    showToast('Logged out successfully', 'success')
    navigate('/login')
  }

  return (
    <div className="min-h-screen bg-background-primary">
      <Navigation />

      <main className="pt-20 pb-8">
        <div className="container-custom max-w-3xl">
          {/* Header */}
          <div className="mb-6">
            <h1 className="text-3xl font-bold text-text-primary mb-2">
              Settings
            </h1>
            <p className="text-text-secondary">
              Manage your account and preferences
            </p>
          </div>

          <div className="space-y-6">
            {/* Profile Section */}
            <Card>
              <div className="flex items-center gap-3 mb-4">
                <User className="w-5 h-5 text-accent" />
                <h2 className="text-xl font-semibold text-text-primary">
                  Profile
                </h2>
              </div>

              <div className="space-y-3">
                <div>
                  <p className="text-sm text-text-tertiary mb-1">Email</p>
                  <p className="text-text-primary">{user?.email}</p>
                </div>

                {user?.username && (
                  <div>
                    <p className="text-sm text-text-tertiary mb-1">Username</p>
                    <p className="text-text-primary">{user.username}</p>
                  </div>
                )}

                <div>
                  <p className="text-sm text-text-tertiary mb-1">Member Since</p>
                  <p className="text-text-primary">
                    {new Date(user?.createdAt || '').toLocaleDateString()}
                  </p>
                </div>
              </div>
            </Card>

            {/* Appearance Section */}
            <Card>
              <div className="flex items-center gap-3 mb-4">
                {theme === 'dark' ? (
                  <Moon className="w-5 h-5 text-accent" />
                ) : (
                  <Sun className="w-5 h-5 text-accent" />
                )}
                <h2 className="text-xl font-semibold text-text-primary">
                  Appearance
                </h2>
              </div>

              <div className="flex items-center justify-between">
                <div>
                  <p className="text-text-primary font-medium">Theme</p>
                  <p className="text-sm text-text-secondary">
                    Current: {theme === 'dark' ? 'Dark' : 'Light'}
                  </p>
                </div>

                <Button variant="secondary" onClick={toggleTheme}>
                  {theme === 'dark' ? (
                    <>
                      <Sun className="w-4 h-4" />
                      Switch to Light
                    </>
                  ) : (
                    <>
                      <Moon className="w-4 h-4" />
                      Switch to Dark
                    </>
                  )}
                </Button>
              </div>
            </Card>

            {/* Notifications Section */}
            <Card>
              <div className="flex items-center gap-3 mb-4">
                <Bell className="w-5 h-5 text-accent" />
                <h2 className="text-xl font-semibold text-text-primary">
                  Notifications
                </h2>
              </div>

              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-text-primary font-medium">
                      Email Notifications
                    </p>
                    <p className="text-sm text-text-secondary">
                      Receive alerts via email
                    </p>
                  </div>
                  <label className="relative inline-flex items-center cursor-pointer">
                    <input
                      type="checkbox"
                      className="sr-only peer"
                      defaultChecked={user?.preferences?.emailNotifications}
                      onChange={(e) =>
                        showToast(
                          e.target.checked
                            ? 'Email notifications enabled'
                            : 'Email notifications disabled',
                          'info'
                        )
                      }
                    />
                    <div className="w-11 h-6 bg-background-tertiary peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-accent rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-border-primary after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-accent"></div>
                  </label>
                </div>

                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-text-primary font-medium">
                      Price Alerts
                    </p>
                    <p className="text-sm text-text-secondary">
                      Notify when price targets are hit
                    </p>
                  </div>
                  <label className="relative inline-flex items-center cursor-pointer">
                    <input
                      type="checkbox"
                      className="sr-only peer"
                      defaultChecked={user?.preferences?.priceAlerts}
                      onChange={(e) =>
                        showToast(
                          e.target.checked
                            ? 'Price alerts enabled'
                            : 'Price alerts disabled',
                          'info'
                        )
                      }
                    />
                    <div className="w-11 h-6 bg-background-tertiary peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-accent rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-border-primary after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-accent"></div>
                  </label>
                </div>
              </div>
            </Card>

            {/* Account Actions */}
            <Card>
              <div className="space-y-3">
                <Button
                  variant="danger"
                  onClick={handleLogout}
                  fullWidth
                >
                  <LogOut className="w-4 h-4" />
                  Logout
                </Button>
              </div>
            </Card>
          </div>
        </div>
      </main>
    </div>
  )
}
