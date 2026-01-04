/**
 * Register Page
 * User registration with email, password, and password confirmation
 */

import React, { useState } from 'react'
import { Link, useNavigate } from 'react-router-dom'
import { TrendingUp, Mail, Lock, User } from 'lucide-react'
import { useAuth, useNotification } from '@/contexts'
import { Button, Input } from '@/components'
import { validateEmail, validatePassword, passwordsMatch, isRequired } from '@/utils'

export function RegisterPage() {
  const { register, isLoading } = useAuth()
  const { showToast } = useNotification()
  const navigate = useNavigate()

  const [formData, setFormData] = useState({
    email: '',
    username: '',
    password: '',
    confirmPassword: '',
  })

  const [errors, setErrors] = useState({
    email: '',
    username: '',
    password: '',
    confirmPassword: '',
  })

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target
    setFormData((prev) => ({ ...prev, [name]: value }))

    // Clear error when user types
    if (errors[name as keyof typeof errors]) {
      setErrors((prev) => ({ ...prev, [name]: '' }))
    }
  }

  const validate = (): boolean => {
    const newErrors = {
      email: '',
      username: '',
      password: '',
      confirmPassword: '',
    }

    if (!isRequired(formData.email)) {
      newErrors.email = 'Email is required'
    } else if (!validateEmail(formData.email)) {
      newErrors.email = 'Please enter a valid email address'
    }

    const passwordValidation = validatePassword(formData.password)
    if (!isRequired(formData.password)) {
      newErrors.password = 'Password is required'
    } else if (!passwordValidation.isValid) {
      newErrors.password = passwordValidation.errors[0]
    }

    if (!isRequired(formData.confirmPassword)) {
      newErrors.confirmPassword = 'Please confirm your password'
    } else if (!passwordsMatch(formData.password, formData.confirmPassword)) {
      newErrors.confirmPassword = 'Passwords do not match'
    }

    setErrors(newErrors)
    return Object.values(newErrors).every((error) => !error)
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!validate()) return

    try {
      await register({
        email: formData.email,
        username: formData.username,
        password: formData.password,
        confirmPassword: formData.confirmPassword,
      })

      showToast('Account created successfully!', 'success')
      navigate('/')
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : 'Registration failed'
      showToast(errorMessage, 'error')
    }
  }

  const passwordStrength = validatePassword(formData.password).strength
  const showPasswordStrength = formData.password.length > 0

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
            Create Account
          </h2>
          <p className="text-text-secondary mb-6">
            Sign up to start tracking your investments
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
              label="Username (Optional)"
              type="text"
              name="username"
              value={formData.username}
              onChange={handleChange}
              error={errors.username}
              icon={User}
              placeholder="Choose a username"
              autoComplete="username"
              disabled={isLoading}
            />

            <div>
              <Input
                label="Password"
                type="password"
                name="password"
                value={formData.password}
                onChange={handleChange}
                error={errors.password}
                icon={Lock}
                placeholder="Create a password"
                autoComplete="new-password"
                disabled={isLoading}
              />

              {showPasswordStrength && !errors.password && (
                <div className="mt-2">
                  <div className="flex items-center gap-2">
                    <div className="flex-1 h-1 bg-background-tertiary rounded overflow-hidden">
                      <div
                        className={`h-full transition-all ${
                          passwordStrength === 'strong'
                            ? 'w-full bg-success'
                            : passwordStrength === 'medium'
                            ? 'w-2/3 bg-warning'
                            : 'w-1/3 bg-danger'
                        }`}
                      />
                    </div>
                    <span
                      className={`text-xs font-medium ${
                        passwordStrength === 'strong'
                          ? 'text-success'
                          : passwordStrength === 'medium'
                          ? 'text-warning'
                          : 'text-danger'
                      }`}
                    >
                      {passwordStrength.charAt(0).toUpperCase() + passwordStrength.slice(1)}
                    </span>
                  </div>
                </div>
              )}
            </div>

            <Input
              label="Confirm Password"
              type="password"
              name="confirmPassword"
              value={formData.confirmPassword}
              onChange={handleChange}
              error={errors.confirmPassword}
              icon={Lock}
              placeholder="Confirm your password"
              autoComplete="new-password"
              disabled={isLoading}
            />

            <Button
              type="submit"
              variant="primary"
              fullWidth
              loading={isLoading}
              disabled={isLoading}
            >
              Create Account
            </Button>
          </form>

          <div className="mt-6 text-center">
            <p className="text-sm text-text-secondary">
              Already have an account?{' '}
              <Link to="/login" className="text-accent hover:text-accent-hover font-medium">
                Sign in
              </Link>
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}
