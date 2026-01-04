# Requirements Document: Stock Price Dashboard

## 1. Project Overview

**Project Name:** My-Invest Dashboard

**Description:** A Progressive Web Application (PWA) that provides real-time and historical stock market data visualization, supporting multiple asset types including stocks, cryptocurrencies, ETFs, and bonds. The application enables users to track their investment watchlists, analyze price trends through interactive charts, and receive alerts based on price movements and pattern detection.

**Target Users:** Individual investors, traders, and financial enthusiasts who need to monitor and analyze market data across different asset classes.

## 2. Technical Stack

### Frontend
- **Framework:** React (latest stable version)
- **Type System:** TypeScript
- **State Management:** TBD (Redux Toolkit, Zustand, or React Context based on complexity)
- **Charting Library:** TradingView Lightweight Charts
- **PWA Support:** Workbox for service workers and offline functionality
- **Styling:** TBD (Tailwind CSS, Material-UI, or styled-components)
- **Build Tool:** Vite or Create React App

### Backend
- **Language/Framework:** Rust
  - Web Framework: Actix-web or Axum
  - Async Runtime: Tokio
- **API Architecture:** RESTful API with potential WebSocket support for real-time updates
- **Authentication:** JWT-based authentication

### Database
- **Primary Database:** MongoDB (for user data, watchlists, alerts configuration, and historical data)
- **Cache/Real-time Data:** Redis (for caching API responses, real-time price data, and session management)

### External Services
- Market data providers (to be integrated):
  - Stock data: Alpha Vantage, Finnhub, or Yahoo Finance API
  - Cryptocurrency data: CoinGecko, Binance API, or CryptoCompare
  - ETF/Bond data: As available from chosen provider

## 3. Functional Requirements

### 3.1 User Authentication & Authorization
- **FR-1.1:** Users must be able to register with email and password
- **FR-1.2:** Users must be able to log in securely
- **FR-1.3:** Users must be able to reset forgotten passwords
- **FR-1.4:** System must maintain user sessions securely
- **FR-1.5:** Users must be able to log out
- **FR-1.6:** User data must be isolated per account

### 3.2 Asset Search & Discovery
- **FR-2.1:** Users must be able to search for assets by:
  - Ticker symbol (e.g., AAPL, BTC-USD)
  - Company/asset name (e.g., "Apple Inc.")
  - Type (Stock, Crypto, ETF, Bond)
- **FR-2.2:** Search results must display:
  - Asset name and ticker
  - Current price
  - Asset type
  - Daily change (percentage and absolute)
- **FR-2.3:** Search must support auto-complete/suggestions
- **FR-2.4:** Users must be able to filter search results by asset type

### 3.3 Watchlist Management
- **FR-3.1:** Users must be able to create custom watchlists
- **FR-3.2:** Users must be able to add assets to watchlists
- **FR-3.3:** Users must be able to remove assets from watchlists
- **FR-3.4:** Users must be able to rename watchlists
- **FR-3.5:** Users must be able to delete watchlists
- **FR-3.6:** Watchlists must persist across sessions
- **FR-3.7:** Users must be able to view multiple watchlists

### 3.4 Price Display & Monitoring
- **FR-4.1:** Dashboard must display a list view of assets showing:
  - Asset name and ticker
  - Current price
  - 24h change (percentage and value)
  - 24h volume
  - Market cap (if applicable)
  - Asset type badge
- **FR-4.2:** Price data must be refreshable (manual or auto-refresh)
- **FR-4.3:** Users must be able to filter displayed assets by type:
  - Stocks
  - Cryptocurrencies
  - ETFs
  - Bonds
  - All (default)
- **FR-4.4:** List must support sorting by:
  - Name
  - Price
  - Change percentage
  - Volume

### 3.5 Chart & Historical Data Visualization
- **FR-5.1:** Users must be able to view interactive price charts for each asset
- **FR-5.2:** Charts must support multiple timeframes:
  - 1 Day (1D)
  - 1 Week (1W)
  - 1 Month (1M)
  - 3 Months (3M)
  - 6 Months (6M)
  - 1 Year (1Y)
  - 5 Years (5Y)
  - All Time (MAX)
- **FR-5.3:** Charts must display:
  - Candlestick or line chart options
  - Volume bars
  - Grid lines and axis labels
  - Crosshair for precise value reading
  - Zoom and pan capabilities
- **FR-5.4:** Chart data must be fetched based on selected timeframe

### 3.6 Price Alerts
- **FR-6.1:** Users must be able to create price alerts with:
  - Target price
  - Condition (above/below/crosses)
  - Alert type (one-time or recurring)
- **FR-6.2:** Users must receive notifications when alert conditions are met
- **FR-6.3:** Alerts must be manageable (view, edit, delete)
- **FR-6.4:** Users must be able to enable/disable alerts without deleting them
- **FR-6.5:** System must check alert conditions periodically
- **FR-6.6:** PWA notifications must be supported

### 3.7 Pattern Alerts
- **FR-7.1:** System must detect common price patterns:
  - Moving average crossovers (Golden Cross, Death Cross)
  - Support/Resistance breakouts
  - RSI overbought/oversold conditions
  - Volume spikes
  - Custom pattern definitions (future enhancement)
- **FR-7.2:** Users must be able to subscribe to pattern alerts for specific assets
- **FR-7.3:** Users must receive notifications when patterns are detected
- **FR-7.4:** Pattern alerts must be configurable (parameters, sensitivity)

### 3.8 Progressive Web App Features
- **FR-8.1:** Application must be installable on desktop and mobile devices
- **FR-8.2:** Application must work offline with cached data
- **FR-8.3:** Application must display appropriate offline indicators
- **FR-8.4:** Application must sync data when connection is restored
- **FR-8.5:** Application manifest must be properly configured

## 4. Non-Functional Requirements

### 4.1 Performance
- **NFR-1.1:** Initial page load must complete within 3 seconds on 3G connection
- **NFR-1.2:** Chart rendering must complete within 1 second
- **NFR-1.3:** Search results must appear within 500ms
- **NFR-1.4:** API responses must be cached appropriately in Redis
- **NFR-1.5:** Real-time price updates should occur at least every 30 seconds

### 4.2 Scalability
- **NFR-2.1:** System must support at least 10,000 concurrent users
- **NFR-2.2:** Database must efficiently handle growing historical data
- **NFR-2.3:** Redis cache must be sized appropriately for all active users

### 4.3 Security
- **NFR-3.1:** All API communications must use HTTPS
- **NFR-3.2:** Passwords must be hashed using bcrypt or Argon2
- **NFR-3.3:** JWT tokens must expire after reasonable time period
- **NFR-3.4:** Rate limiting must be implemented on all API endpoints
- **NFR-3.5:** Input validation must be performed on all user inputs
- **NFR-3.6:** XSS and CSRF protections must be implemented

### 4.4 Reliability
- **NFR-4.1:** System uptime must be at least 99.5%
- **NFR-4.2:** Data must be backed up regularly
- **NFR-4.3:** Graceful error handling must be implemented throughout

### 4.5 Usability
- **NFR-5.1:** UI must be responsive (mobile, tablet, desktop)
- **NFR-5.2:** Application must be accessible (WCAG 2.1 Level AA)
- **NFR-5.3:** Color scheme must support light and dark modes
- **NFR-5.4:** Interface must be intuitive requiring minimal learning

### 4.6 Maintainability
- **NFR-6.1:** Code must follow language-specific best practices
- **NFR-6.2:** API must be versioned
- **NFR-6.3:** Comprehensive logging must be implemented
- **NFR-6.4:** Code must include unit and integration tests

## 5. System Architecture

### 5.1 High-Level Architecture
```
┌─────────────────────────────────────────────┐
│          Client (React PWA)                 │
│  - TradingView Charts                       │
│  - Service Worker (offline support)         │
└──────────────────┬──────────────────────────┘
                   │ HTTPS/WSS
                   │
┌──────────────────▼──────────────────────────┐
│       Backend API (Rust)                    │
│  - Actix-web/Axum                           │
│  - JWT Authentication                       │
│  - WebSocket for real-time                  │
│  - Pattern Detection Service                │
│  - Alert Processing Service                 │
└────┬──────────────────────┬─────────────────┘
     │                      │
     │                      │
┌────▼────────┐      ┌──────▼──────────┐
│   MongoDB   │      │     Redis       │
│  - Users    │      │  - Cache        │
│  - Watchlists│     │  - Sessions     │
│  - Alerts   │      │  - Real-time    │
│  - History  │      │    prices       │
└─────────────┘      └─────────────────┘
     │
     │
┌────▼─────────────────────────────────┐
│   External Market Data APIs          │
│  - Stock APIs                         │
│  - Crypto APIs                        │
│  - ETF/Bond data providers            │
└───────────────────────────────────────┘
```

### 5.2 Backend Services
- **API Service:** RESTful endpoints for CRUD operations
- **WebSocket Service:** Real-time price updates
- **Alert Processor:** Background job to check price and pattern alerts
- **Data Aggregator:** Fetch and normalize data from multiple sources
- **Authentication Service:** Handle JWT generation and validation

## 6. Data Models

### 6.1 User
```typescript
{
  _id: ObjectId,
  email: string,
  passwordHash: string,
  createdAt: DateTime,
  updatedAt: DateTime,
  preferences: {
    theme: "light" | "dark",
    defaultTimeframe: string,
    notifications: boolean
  }
}
```

### 6.2 Watchlist
```typescript
{
  _id: ObjectId,
  userId: ObjectId,
  name: string,
  assets: [
    {
      symbol: string,
      assetType: "stock" | "crypto" | "etf" | "bond",
      addedAt: DateTime
    }
  ],
  createdAt: DateTime,
  updatedAt: DateTime
}
```

### 6.3 Alert
```typescript
{
  _id: ObjectId,
  userId: ObjectId,
  symbol: string,
  alertType: "price" | "pattern",

  // For price alerts
  targetPrice?: number,
  condition?: "above" | "below" | "crosses",
  isRecurring?: boolean,

  // For pattern alerts
  patternType?: string,
  patternConfig?: object,

  isActive: boolean,
  triggeredAt?: DateTime,
  createdAt: DateTime
}
```

### 6.4 Asset (cached in Redis)
```typescript
{
  symbol: string,
  name: string,
  assetType: "stock" | "crypto" | "etf" | "bond",
  currentPrice: number,
  change24h: number,
  changePercent24h: number,
  volume24h: number,
  marketCap?: number,
  lastUpdated: DateTime
}
```

## 7. API Endpoints

### 7.1 Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login user
- `POST /api/v1/auth/logout` - Logout user
- `POST /api/v1/auth/refresh` - Refresh JWT token
- `POST /api/v1/auth/reset-password` - Request password reset
- `POST /api/v1/auth/reset-password/confirm` - Confirm password reset

### 7.2 Assets
- `GET /api/v1/assets/search?q={query}&type={type}` - Search assets
- `GET /api/v1/assets/{symbol}` - Get asset details
- `GET /api/v1/assets/{symbol}/history?timeframe={timeframe}` - Get historical data

### 7.3 Watchlists
- `GET /api/v1/watchlists` - Get user's watchlists
- `POST /api/v1/watchlists` - Create watchlist
- `GET /api/v1/watchlists/{id}` - Get specific watchlist
- `PUT /api/v1/watchlists/{id}` - Update watchlist
- `DELETE /api/v1/watchlists/{id}` - Delete watchlist
- `POST /api/v1/watchlists/{id}/assets` - Add asset to watchlist
- `DELETE /api/v1/watchlists/{id}/assets/{symbol}` - Remove asset from watchlist

### 7.4 Alerts
- `GET /api/v1/alerts` - Get user's alerts
- `POST /api/v1/alerts` - Create alert
- `GET /api/v1/alerts/{id}` - Get specific alert
- `PUT /api/v1/alerts/{id}` - Update alert
- `DELETE /api/v1/alerts/{id}` - Delete alert
- `PATCH /api/v1/alerts/{id}/toggle` - Enable/disable alert

### 7.5 WebSocket
- `WS /api/v1/ws` - WebSocket connection for real-time price updates
  - Subscribe: `{"action": "subscribe", "symbols": ["AAPL", "BTC-USD"]}`
  - Unsubscribe: `{"action": "unsubscribe", "symbols": ["AAPL"]}`
  - Receive: `{"symbol": "AAPL", "price": 150.25, "change": 1.5, "timestamp": "..."}`

## 8. User Interface Requirements

### 8.1 Pages/Views
1. **Login/Register Page**
   - Email/password form
   - Password reset link
   - Social login (optional future enhancement)

2. **Dashboard (Main View)**
   - Top navigation bar with watchlist selector and filters
   - Asset list table/grid
   - Quick stats/summary cards
   - Filter chips (All, Stock, Crypto, ETF, Bond)
   - Search bar

3. **Asset Detail View**
   - Large interactive chart
   - Asset information panel
   - Add to watchlist button
   - Create alert button
   - Historical data table

4. **Watchlist Management**
   - List of watchlists
   - Create/edit/delete watchlist
   - Drag-and-drop reordering (optional)

5. **Alerts Management**
   - List of all alerts
   - Create new alert form
   - Alert history/triggered alerts

6. **Settings/Profile**
   - User profile information
   - Notification preferences
   - Theme selection
   - Default chart settings

### 8.2 UI Components
- Navigation bar with user menu
- Asset card/row component
- Price chart component (TradingView integration)
- Search input with autocomplete
- Alert creation modal/form
- Notification toast/banner
- Loading skeletons
- Error boundaries
- Offline indicator

### 8.3 Responsive Breakpoints
- Mobile: < 640px
- Tablet: 640px - 1024px
- Desktop: > 1024px

## 9. Development Phases

### Phase 1: MVP (Minimum Viable Product)
- User authentication
- Basic watchlist management
- Asset search (limited to stocks only)
- Price list display
- Simple line chart with basic timeframes
- Manual price refresh

### Phase 2: Enhanced Features
- Add Crypto, ETF, Bond support
- Advanced filtering and sorting
- Price alerts (basic above/below)
- Auto-refresh price data
- Historical data views (all timeframes)
- TradingView chart integration
- PWA basic features (installability)

### Phase 3: Advanced Features
- Pattern detection and alerts
- WebSocket real-time updates
- Advanced chart features (indicators, overlays)
- Full PWA support with offline mode
- Notification system
- Performance optimizations

### Phase 4: Polish & Scale
- Dark mode
- Mobile optimizations
- Advanced search features
- Analytics and usage tracking
- Performance monitoring
- Load testing and optimization

## 10. Testing Requirements

### 10.1 Frontend Testing
- Unit tests for React components (Jest + React Testing Library)
- Integration tests for user flows
- E2E tests for critical paths (Playwright or Cypress)
- Chart rendering tests
- PWA functionality tests

### 10.2 Backend Testing
- Unit tests for business logic
- Integration tests for API endpoints
- Database operation tests
- WebSocket connection tests
- Alert processing tests
- Load/performance tests

### 10.3 Security Testing
- Authentication flow testing
- Authorization tests
- Input validation tests
- Rate limiting tests
- OWASP top 10 vulnerability checks

## 11. Deployment Requirements

### 11.1 Local Development Environment
- **Docker Compose Setup:** Complete local development environment with all services
  - Frontend container (React dev server with hot reload)
  - Backend container (Rust API server with auto-recompile)
  - MongoDB container (with persistent volumes)
  - Redis container (with persistent volumes)
  - Volume mounts for live code reloading during development
  - Network configuration for inter-service communication
  - Health checks for all services
- **Requirements:**
  - Docker and Docker Compose installed
  - Single `docker-compose up` command to start entire stack
  - Environment variables configured via `.env` file
  - Separate `docker-compose.dev.yml` and `docker-compose.prod.yml` configurations
  - Database seeding scripts for local development data

### 11.2 Cloud Infrastructure (Google Cloud Platform)
**Primary Goal:** Minimize costs using GCP Free Tier and cost-effective services

- **Frontend Hosting:**
  - **Option 1 (Recommended):** Cloud Storage (static hosting) + Cloud CDN
    - Free Tier: 5GB storage, 1GB network egress/month, 5,000 Class A operations
    - Estimated cost after free tier: ~$0.01-0.05/month for small traffic
  - **Option 2:** Cloud Run (containerized frontend with SSR if needed)
    - Free Tier: 2 million requests/month, 360,000 GB-seconds
    - Auto-scales to zero (no idle costs)

- **Backend API:**
  - Google Cloud Run (containerized Rust application)
  - Free Tier: 2 million requests/month, 360,000 GB-seconds, 180,000 vCPU-seconds
  - Autoscaling from 0 to N instances based on traffic
  - Only pay for actual request processing time
  - Estimated cost: $0 for low traffic (within free tier)

- **Database (MongoDB):**
  - **Option 1 (Recommended):** MongoDB Atlas M0 Free Tier
    - 512MB storage, shared cluster, always free
    - Suitable for development and small production workloads
  - **Option 2:** Self-hosted on GCP Compute Engine e2-micro
    - Free Tier: 1 e2-micro instance per month (US regions only)
    - 30GB standard persistent disk free

- **Cache (Redis):**
  - **Option 1 (Recommended):** Upstash Redis (serverless)
    - Free Tier: 10,000 commands/day, 256MB storage
    - Pay-as-you-go pricing after free tier
  - **Option 2:** Cloud Memorystore for Redis
    - No free tier, starts at ~$35/month for basic tier
    - Only use if high performance is critical
  - **Option 3 (Dev/Low Traffic):** Redis in Cloud Run sidecar container

- **Infrastructure as Code (Terraform):**
  - All GCP resources provisioned via Terraform
  - Terraform Cloud (free tier for up to 5 users)
  - Separate workspaces: `dev`, `staging`, `production`
  - Remote state stored in GCS bucket (free tier: 5GB storage)
  - Modular structure for reusability
  - Version-controlled infrastructure

- **Additional Services:**
  - **Container Registry:** Google Artifact Registry (500MB free storage/month)
  - **Secret Management:** Google Secret Manager (6 active secret versions free)
  - **Logging:** Cloud Logging (free tier: 50GB/month)
  - **Monitoring:** Cloud Monitoring (free tier included)

### 11.3 CI/CD Pipeline
- **Platform:** GitHub Actions (2,000 free minutes/month for private repos)
- **Pipeline Stages:**
  1. Automated linting and code quality checks
  2. Unit and integration tests
  3. Build Docker images for frontend and backend
  4. Push images to Google Artifact Registry
  5. Terraform plan (on PR) / Terraform apply (on merge)
  6. Deploy to Cloud Run (backend) and Cloud Storage/CDN (frontend)
  7. Run smoke tests against deployed environment
  8. Database migrations (automated with rollback capability)
- **Environments:**
  - Development: Auto-deploy on merge to `develop` branch
  - Staging: Auto-deploy on merge to `staging` branch
  - Production: Manual approval required for `main` branch deployments
- **Secrets Management:**
  - GitHub Secrets for CI/CD credentials
  - GCP Secret Manager for application secrets
  - Terraform variables for environment-specific configuration

### 11.4 Monitoring & Observability
- **Application Performance Monitoring:**
  - Cloud Monitoring (GCP native, free tier included)
  - Cloud Trace for distributed tracing (free tier: 2.5M trace spans/month)
- **Error Tracking:**
  - Cloud Error Reporting (free with Cloud Logging)
  - Alternative: Sentry (10,000 events/month free tier)
- **Uptime Monitoring:**
  - Cloud Monitoring uptime checks (free tier: 1M API calls/month)
  - Alternative: UptimeRobot (50 monitors free)
- **Logging:**
  - Cloud Logging (free tier: 50GB/month)
  - Structured logging in JSON format
  - Log retention policies to control costs
- **Cost Monitoring:**
  - GCP Budget alerts to prevent unexpected charges
  - Monthly cost reports and analysis

## 12. Future Enhancements (Out of Scope for V1)
- Portfolio tracking (track actual holdings)
- News integration
- Social features (share watchlists)
- Advanced technical indicators
- Backtesting capabilities
- Multi-language support
- Mobile native apps (React Native)
- AI-powered insights and predictions
- Compare multiple assets side-by-side
- Export data to CSV/Excel
- Integration with broker APIs
