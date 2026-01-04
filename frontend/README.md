# My-Invest Frontend MVP

Progressive Web Application for tracking and analyzing stock market data.

## Quick Start with Docker

### Development Mode

```bash
# Start development server with hot reload
docker-compose -f docker-compose.dev.yml up

# Access at http://localhost:3000
```

### Production Mode

```bash
# Build and run production build
docker-compose up --build

# Access at http://localhost:3000
```

### Stop Services

```bash
# Development
docker-compose -f docker-compose.dev.yml down

# Production
docker-compose down
```

## Local Development (without Docker)

```bash
cd frontend
npm install
npm run dev
```

## Features

✅ Authentication (mock with localStorage)
✅ Dashboard with asset grid (Stocks, Crypto, ETFs, Bonds)
✅ Asset detail with TradingView charts
✅ Watchlist management
✅ Settings (theme, notifications, profile)
✅ Dark/Light theme toggle
✅ Progressive Web App (PWA)
✅ Fully responsive design

## Tech Stack

- React 18.2, TypeScript 5.3, Vite 5.0
- Tailwind CSS 3.4, TradingView Charts 4.1
- React Router 6.21, Lucide React
- Vite PWA Plugin (Service Worker)

## Build Stats

- Total bundle: ~401 KiB (gzipped)
- React vendor: 162 KiB
- Chart vendor: 162 KiB
- App code: 60 KiB
- CSS: 24 KiB

## Project Structure

```
frontend/
├── src/
│   ├── components/     # UI components
│   ├── pages/          # Page components
│   ├── contexts/       # React contexts
│   ├── services/       # API services
│   ├── types/          # TypeScript types
│   ├── utils/          # Utilities
│   └── styles/         # Global styles
├── public/             # Static assets
├── Dockerfile          # Production build
├── Dockerfile.dev      # Development build
└── nginx.conf          # Nginx config
```

## Demo Account

Since this is a mock authentication system, you can use any email/password to:
- Create a new account (Register)
- Login with created credentials

All data is stored in browser localStorage.

## Next Steps

1. Add PWA icons to `/frontend/public/`
2. Connect to real backend API (replace mock auth)
3. Add unit/integration tests
4. Set up CI/CD pipeline

## License

[License type to be determined]
