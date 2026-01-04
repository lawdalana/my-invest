/**
 * Login Page
 * User login with email and password
 */

import React, { useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { TrendingUp, Mail, Lock } from 'lucide-react'
import { useAuth, useNotification } from '@/contexts'
import { Button, Input } from '@/components'
import { validateEmail, isRequired } from '@/utils'

export function LoginPage() {
  const { login, isLoading } = useAuth()
  const { showToast } = useNotification()
  const navigate = useNavigate()

  const [formData, setFormData] = useState({
    email: '',
    password: '',
    rememberMe: false,
  })

  const [errors, setErrors] = useState({
    email: '',
    password: '',
  })

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type, checked } = e.target
    setFormData((prev) => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value,
    }))

    // Clear error when user types
    if (errors[name as keyof typeof errors]) {
      setErrors((prev) => ({ ...prev, [name]: '' }))
    }
  }

  const validate = (): boolean => {
    const newErrors = {
      email: '',
      password: '',
    }

    if (!isRequired(formData.email)) {
      newErrors.email = 'Email is required'
    } else if (!validateEmail(formData.email)) {
      newErrors.email = 'Please enter a valid email address'
    }

    if (!isRequired(formData.password)) {
      newErrors.password = 'Password is required'
    }

    setErrors(newErrors)
    return !newErrors.email && !newErrors.password
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!validate()) return

    try {
      await login({
        email: formData.email,
        password: formData.password,
        rememberMe: formData.rememberMe,
      })

      showToast('Login successful!', 'success')
      navigate('/')
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : 'Login failed'
      showToast(errorMessage, 'error')
    }
  }

  return (
    <div className="min-h-screen bg-background-primary flex items-center justify-center p-4">
      <div className="w-full max-w-md">
        {/* Logo */}
        <div className="flex items-center justify-center gap-2 mb-8">
          <TrendingUp className="w-10 h-10 text-accent" />
          <h1 className="text-3xl font-bold text-text-primary">My-Invest</h1>
        </div>

        {/* Form Card */}
        <div className="bg-background-secondary border border-border-primary rounded-lg p-8 shadow-card">
          <h2 className="text-2xl font-semibold text-text-primary mb-2">
            Welcome Back
          </h2>
          <p className="text-text-secondary mb-6">
            Sign in to your account to continue
          </p>

          <form onSubmit={handleSubmit} className="space-y-4">
            <Input
              label="Email"
              type="email"
              name="email"
              value={formData.email}
              onChange={handleChange}
              error={errors.email}
              icon={Mail}
              placeholder="Enter your email"
              autoComplete="email"
              disabled={isLoading}
            />

            <Input
              label="Password"
              type="password"
              name="password"
              value={formData.password}
              onChange={handleChange}
              error={errors.password}
              icon={Lock}
              placeholder="Enter your password"
              autoComplete="current-password"
              disabled={isLoading}
            />

            <div className="flex items-center justify-between">
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  name="rememberMe"
                  checked={formData.rememberMe}
                  onChange={handleChange}
                  disabled={isLoading}
                  className="w-4 h-4 rounded border-border-primary bg-background-tertiary text-accent focus:ring-accent focus:ring-offset-background-secondary"
                />
                <span className="text-sm text-text-secondary">
                  Remember me
                </span>
              </label>
            </div>

            <Button
              type="submit"
              variant="primary"
              fullWidth
              loading={isLoading}
              disabled={isLoading}
            >
              Sign In
            </Button>
          </form>

          <div className="mt-6 text-center">
            <p className="text-sm text-text-secondary">
              Don't have an account?{' '}
              <Link to="/register" className="text-accent hover:text-accent-hover font-medium">
                Sign up
              </Link>
            </p>
          </div>
        </div>

        {/* Demo Info */}
        <div className="mt-6 p-4 bg-accent-light border border-accent/20 rounded text-center">
          <p className="text-sm text-text-secondary">
            <strong className="text-text-primary">Demo Mode:</strong> Use any email/password to create an account
          </p>
        </div>
      </div>
    </div>
  )
}
