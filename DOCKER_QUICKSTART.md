# Docker Compose Quick Start Guide

This guide shows you how to run the entire My-Invest Dashboard stack (frontend + backend + databases) using Docker Compose.

## Prerequisites

- Docker installed ([Get Docker](https://docs.docker.com/get-docker/))
- Docker Compose installed (included with Docker Desktop)
- Alpha Vantage API key ([Get free API key](https://www.alphavantage.co/support/#api-key))

## Quick Start (Development)

### 1. Set Up Environment Variables

Create a `.env` file in the project root (optional for development):

```bash
# Alpha Vantage API Key (required)
ALPHA_VANTAGE_API_KEY=your_api_key_here

# JWT Secret (auto-generated in dev if not provided)
# JWT_SECRET=your-secret-key

# Other variables use defaults in development
```

Or set the API key directly:

```bash
export ALPHA_VANTAGE_API_KEY=your_api_key_here
```

### 2. Start All Services

```bash
# Start the entire stack (frontend, backend, MongoDB, Redis)
docker-compose -f docker-compose.dev.yml up

# Or start in detached mode (runs in background)
docker-compose -f docker-compose.dev.yml up -d
```

### 3. Access the Application

Once all services are running (wait ~60 seconds for backend to compile):

- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:8080
- **API Health Check**: http://localhost:8080/health
- **MongoDB**: mongodb://localhost:27017
- **Redis**: redis://localhost:6379

### 4. View Logs

```bash
# View all service logs
docker-compose -f docker-compose.dev.yml logs -f

# View specific service logs
docker-compose -f docker-compose.dev.yml logs -f backend
docker-compose -f docker-compose.dev.yml logs -f frontend
```

### 5. Stop Services

```bash
# Stop and remove containers
docker-compose -f docker-compose.dev.yml down

# Stop and remove containers + volumes (deletes database data)
docker-compose -f docker-compose.dev.yml down -v
```

## Services Overview

### Development Stack (`docker-compose.dev.yml`)

| Service | Port | Description |
|---------|------|-------------|
| **frontend** | 3000 | React development server with hot reload |
| **backend** | 8080 | Rust API server with cargo-watch auto-rebuild |
| **mongodb** | 27017 | MongoDB 7.0 database (no authentication in dev) |
| **redis** | 6379 | Redis 7.2 cache |

**Development Features:**
- ✅ Hot reload for both frontend and backend
- ✅ Source code mounted as volumes (changes reflect immediately)
- ✅ Debug logging enabled
- ✅ No authentication required for databases
- ✅ Cargo cache persisted for faster rebuilds

### Production Stack (`docker-compose.yml`)

| Service | Port | Description |
|---------|------|-------------|
| **frontend** | 3000 (nginx on 80) | Optimized production build served by Nginx |
| **backend** | 8080 | Rust release binary with optimizations |
| **mongodb** | 27017 | MongoDB with authentication |
| **redis** | 6379 | Redis with password protection |

**Production Features:**
- ✅ Optimized builds (multi-stage Docker builds)
- ✅ Authentication required for databases
- ✅ Health checks for all services
- ✅ Automatic restart on failure
- ✅ Production logging levels

## Optional Development Tools

You can enable additional GUI tools for database management by uncommenting sections in `docker-compose.dev.yml`:

### Redis Commander (Redis GUI)

Uncomment the `redis-commander` service to access Redis GUI at http://localhost:8081

### Mongo Express (MongoDB GUI)

Uncomment the `mongo-express` service to access MongoDB GUI at http://localhost:8082
- Username: `admin`
- Password: `admin`

## Common Commands

### Development Workflow

```bash
# Start fresh (rebuild images)
docker-compose -f docker-compose.dev.yml up --build

# Restart a specific service
docker-compose -f docker-compose.dev.yml restart backend

# Execute command in running container
docker-compose -f docker-compose.dev.yml exec backend sh

# View container status
docker-compose -f docker-compose.dev.yml ps
```

### Database Management

```bash
# Access MongoDB shell
docker-compose -f docker-compose.dev.yml exec mongodb mongosh

# Access Redis CLI
docker-compose -f docker-compose.dev.yml exec redis redis-cli

# Backup MongoDB
docker-compose -f docker-compose.dev.yml exec mongodb mongodump -o /data/backup

# Clear Redis cache
docker-compose -f docker-compose.dev.yml exec redis redis-cli FLUSHALL
```

### Cleanup

```bash
# Remove stopped containers
docker-compose -f docker-compose.dev.yml rm

# Remove unused images
docker image prune -a

# Remove all volumes (WARNING: deletes all database data)
docker-compose -f docker-compose.dev.yml down -v
```

## Production Deployment

### 1. Set Production Environment Variables

Create a `.env` file with production values:

```bash
# Required
JWT_SECRET=your-very-secure-secret-key-change-this
ALPHA_VANTAGE_API_KEY=your_production_api_key
MONGO_ROOT_PASSWORD=your-secure-mongo-password
REDIS_PASSWORD=your-secure-redis-password

# Optional
MONGODB_URI=mongodb://mongodb:27017
MONGODB_DATABASE=my_invest
ALLOWED_ORIGINS=https://yourdomain.com
```

### 2. Start Production Stack

```bash
# Build and start production services
docker-compose up -d --build

# View logs
docker-compose logs -f

# Check service health
docker-compose ps
```

### 3. Production Best Practices

- ✅ Use strong, unique passwords for all services
- ✅ Enable MongoDB authentication
- ✅ Use Redis password protection
- ✅ Set up SSL/TLS termination (use reverse proxy like Nginx or Traefik)
- ✅ Configure backup strategy for MongoDB
- ✅ Monitor logs and metrics
- ✅ Set up log rotation
- ✅ Use Docker secrets for sensitive data

## Troubleshooting

### Backend Fails to Start

**Issue**: Backend exits with "connection refused" error

**Solution**: Wait for MongoDB and Redis to be healthy (~30 seconds), then restart:
```bash
docker-compose -f docker-compose.dev.yml restart backend
```

### Frontend Can't Reach Backend

**Issue**: Frontend shows "Network Error" or "Cannot connect to API"

**Solutions**:
1. Check backend is healthy: `curl http://localhost:8080/health`
2. Verify VITE_API_BASE_URL in frontend environment variables
3. Check CORS settings in backend allow `http://localhost:3000`

### MongoDB Permission Denied

**Issue**: MongoDB container fails with permission error

**Solution**: Fix volume permissions:
```bash
docker-compose -f docker-compose.dev.yml down -v
sudo chown -R $(whoami) ./data  # if using local volumes
docker-compose -f docker-compose.dev.yml up
```

### Port Already in Use

**Issue**: "Port 3000 (or 8080, 27017, 6379) is already in use"

**Solutions**:
1. Stop conflicting services
2. Change port mapping in docker-compose file:
   ```yaml
   ports:
     - "3001:3000"  # Use different external port
   ```

### Slow Backend Compilation

**Issue**: Backend takes 5+ minutes to start in development

**Solutions**:
1. Cargo cache volume is working (check with `docker volume ls`)
2. Increase Docker memory allocation (Docker Desktop → Settings → Resources)
3. Use sccache for faster compilation (advanced)

## Performance Tips

### Development

- **Faster backend rebuilds**: Cargo cache is persisted in a volume
- **Instant frontend updates**: Hot module replacement (HMR) enabled
- **Database performance**: Use volumes on SSD
- **Network**: All services on same Docker network (minimal latency)

### Production

- **Multi-stage builds**: Minimal image sizes
- **Layer caching**: Rebuild only changed layers
- **Resource limits**: Add CPU/memory limits for stability
- **Health checks**: Automatic recovery from failures

## Next Steps

1. ✅ Start the stack: `docker-compose -f docker-compose.dev.yml up`
2. ✅ Access frontend: http://localhost:3000
3. ✅ Register a new account
4. ✅ Search for stocks (e.g., AAPL, MSFT, GOOGL)
5. ✅ Create watchlists
6. ✅ View price charts

## Getting Help

- **Backend Issues**: Check `/backend/README.md`
- **Frontend Issues**: Check `/frontend/README.md`
- **Docker Issues**: Run `docker-compose logs -f <service>`
- **API Documentation**: See `/docs/api/phase1-endpoints.md`

---

**Happy coding!** 🚀
