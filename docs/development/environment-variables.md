# Environment Variables Guide

This document describes all environment variables used by the My-Invest backend.

## Quick Setup

```bash
# Copy the example file
cp backend/.env.example backend/.env

# Edit the file with your values
# At minimum, set JWT_SECRET and ALPHA_VANTAGE_API_KEY
```

## Required Variables

These variables must be set for the application to start:

| Variable | Description | Example |
|----------|-------------|---------|
| `JWT_SECRET` | Secret key for signing JWT tokens. Use a long, random string. | `openssl rand -base64 64` |
| `ALPHA_VANTAGE_API_KEY` | Your Alpha Vantage API key | Get from [alphavantage.co](https://www.alphavantage.co/support/#api-key) |

## All Variables

### Server Configuration

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `SERVER_HOST` | Host address to bind to | `0.0.0.0` | No |
| `SERVER_PORT` | Port number | `8080` | No |
| `ENVIRONMENT` | Environment name (`development`, `staging`, `production`) | `development` | No |

### Logging

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `RUST_LOG` | Log level filter | `info,my_invest_backend=debug,tower_http=debug` | No |

Log level options: `trace`, `debug`, `info`, `warn`, `error`

### JWT Authentication

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `JWT_SECRET` | Secret key for JWT signing | - | **Yes** |
| `JWT_ACCESS_EXPIRATION` | Access token lifetime (seconds) | `3600` (1 hour) | No |
| `JWT_REFRESH_EXPIRATION` | Refresh token lifetime (seconds) | `2592000` (30 days) | No |
| `JWT_ISSUER` | Token issuer claim | `my-invest-api` | No |

### MongoDB

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `MONGODB_URI` | MongoDB connection string | `mongodb://localhost:27017` | No |
| `MONGODB_DATABASE` | Database name | `my_invest` | No |
| `MONGODB_MIN_POOL_SIZE` | Minimum connections | `5` | No |
| `MONGODB_MAX_POOL_SIZE` | Maximum connections | `20` | No |

### Redis

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `REDIS_URL` | Redis connection URL | `redis://localhost:6379` | No |
| `REDIS_CACHE_TTL` | Default cache TTL (seconds) | `300` (5 min) | No |
| `REDIS_HISTORY_CACHE_TTL` | Historical data cache TTL | `3600` (1 hour) | No |

### Alpha Vantage API

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `ALPHA_VANTAGE_API_KEY` | API key | - | **Yes** |
| `ALPHA_VANTAGE_BASE_URL` | API base URL | `https://www.alphavantage.co/query` | No |
| `ALPHA_VANTAGE_RATE_LIMIT` | Requests per minute | `5` | No |
| `ALPHA_VANTAGE_DAILY_LIMIT` | Daily request limit | `500` | No |

### Rate Limiting

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `RATE_LIMIT_REQUESTS` | Max requests per window | `100` | No |
| `RATE_LIMIT_WINDOW` | Window size (seconds) | `60` | No |

### CORS

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `ALLOWED_ORIGINS` | Comma-separated allowed origins | `http://localhost:3000,http://localhost:5173` | No |

### Application Settings

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `MAX_WATCHLISTS_PER_USER` | Maximum watchlists per user | `20` | No |
| `MAX_ASSETS_PER_WATCHLIST` | Maximum assets per watchlist | `50` | No |
| `MIN_PASSWORD_LENGTH` | Minimum password length | `8` | No |

## Environment-Specific Examples

### Development

```env
# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
ENVIRONMENT=development

# Logging (verbose for development)
RUST_LOG=debug,my_invest_backend=debug,tower_http=debug

# JWT (shorter expiration for testing)
JWT_SECRET=dev-secret-not-for-production
JWT_ACCESS_EXPIRATION=3600
JWT_REFRESH_EXPIRATION=86400

# Local databases
MONGODB_URI=mongodb://localhost:27017
MONGODB_DATABASE=my_invest_dev
REDIS_URL=redis://localhost:6379

# Alpha Vantage
ALPHA_VANTAGE_API_KEY=your-api-key

# CORS (allow frontend dev servers)
ALLOWED_ORIGINS=http://localhost:3000,http://localhost:5173
```

### Production

```env
# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
ENVIRONMENT=production

# Logging (less verbose)
RUST_LOG=info,my_invest_backend=info

# JWT (secure settings)
JWT_SECRET=<long-random-secret-from-openssl>
JWT_ACCESS_EXPIRATION=3600
JWT_REFRESH_EXPIRATION=2592000

# Production databases
MONGODB_URI=mongodb+srv://user:pass@cluster.mongodb.net
MONGODB_DATABASE=my_invest_prod
REDIS_URL=rediss://default:password@redis-host:6379

# Alpha Vantage
ALPHA_VANTAGE_API_KEY=your-production-key

# CORS (only production domain)
ALLOWED_ORIGINS=https://myapp.example.com
```

### Docker Compose

When using Docker Compose, variables can be set in the compose file or in a `.env` file:

```yaml
# docker-compose.dev.yml
services:
  backend:
    environment:
      SERVER_HOST: "0.0.0.0"
      MONGODB_URI: "mongodb://mongodb:27017"
      REDIS_URL: "redis://redis:6379"
      # ... other variables
```

Or use a `.env` file:

```bash
# Create .env in project root
ALPHA_VANTAGE_API_KEY=your-api-key
```

Docker Compose automatically loads `.env` files.

## Generating Secrets

### JWT Secret

```bash
# Using OpenSSL
openssl rand -base64 64

# Using Python
python3 -c "import secrets; print(secrets.token_urlsafe(64))"

# Using Node.js
node -e "console.log(require('crypto').randomBytes(64).toString('base64'))"
```

### Best Practices

1. **Never commit secrets** - Add `.env` to `.gitignore`
2. **Use different secrets per environment** - Dev, staging, prod should have different keys
3. **Rotate secrets periodically** - Especially JWT_SECRET in production
4. **Use secret management** - Consider HashiCorp Vault, AWS Secrets Manager, or GCP Secret Manager for production

## Troubleshooting

### Variable Not Found

If the application fails to start with "JWT_SECRET must be set":

1. Ensure `.env` file exists in the backend directory
2. Check the variable is not commented out
3. Verify there are no typos in variable names

### MongoDB Connection Failed

Check `MONGODB_URI`:
- Local: `mongodb://localhost:27017`
- Docker: `mongodb://mongodb:27017` (service name)
- Atlas: `mongodb+srv://user:pass@cluster.mongodb.net`

### Redis Connection Failed

Check `REDIS_URL`:
- Local: `redis://localhost:6379`
- Docker: `redis://redis:6379` (service name)
- With password: `redis://:password@host:6379`
- With TLS: `rediss://...` (note the extra 's')

### CORS Errors

Ensure `ALLOWED_ORIGINS` includes your frontend URL:
- No trailing slashes
- Include protocol (http:// or https://)
- Comma-separated, no spaces
