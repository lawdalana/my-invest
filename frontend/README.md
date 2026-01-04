# Frontend - React PWA

Progressive Web Application built with React and TypeScript.

## Directory Structure

```
frontend/
├── src/
│   ├── components/        # Reusable UI components
│   │   ├── common/        # Common components (Button, Input, Card, etc.)
│   │   ├── charts/        # TradingView chart components
│   │   ├── assets/        # Asset-related components (AssetCard, AssetRow)
│   │   ├── watchlist/     # Watchlist components
│   │   ├── alerts/        # Alert components
│   │   └── layout/        # Layout components (Navbar, Sidebar, Footer)
│   ├── pages/             # Page components (routes)
│   │   ├── auth/          # Login, Register, ResetPassword
│   │   ├── dashboard/     # Main dashboard page
│   │   ├── asset-detail/  # Asset detail page with charts
│   │   ├── watchlists/    # Watchlist management
│   │   ├── alerts/        # Alert management
│   │   └── settings/      # User settings
│   ├── services/          # API clients and external services
│   │   ├── api/           # API client configuration
│   │   ├── auth.service.ts
│   │   ├── assets.service.ts
│   │   ├── watchlist.service.ts
│   │   ├── alerts.service.ts
│   │   └── websocket.service.ts
│   ├── hooks/             # Custom React hooks
│   │   ├── useAuth.ts
│   │   ├── useWebSocket.ts
│   │   ├── useAssets.ts
│   │   └── useAlerts.ts
│   ├── contexts/          # React Context providers
│   │   ├── AuthContext.tsx
│   │   ├── ThemeContext.tsx
│   │   └── NotificationContext.tsx
│   ├── utils/             # Utility functions
│   │   ├── formatters.ts  # Price, date formatters
│   │   ├── validators.ts  # Form validation
│   │   └── constants.ts   # App constants
│   ├── types/             # TypeScript type definitions
│   │   ├── api.types.ts
│   │   ├── asset.types.ts
│   │   ├── user.types.ts
│   │   └── alert.types.ts
│   ├── assets/            # Static assets
│   │   ├── images/
│   │   └── icons/
│   ├── styles/            # Global styles and theme
│   │   ├── globals.css
│   │   └── theme.ts
│   ├── App.tsx            # Main App component
│   ├── main.tsx           # Entry point
│   └── service-worker.ts  # PWA service worker
├── public/                # Public static files
│   ├── manifest.json      # PWA manifest
│   ├── icons/             # PWA icons (various sizes)
│   └── index.html
├── tests/                 # Test files
│   ├── unit/              # Unit tests (Jest + RTL)
│   ├── integration/       # Integration tests
│   └── e2e/               # End-to-end tests (Playwright/Cypress)
├── .env.example           # Environment variables example
├── .eslintrc.json         # ESLint configuration
├── .prettierrc            # Prettier configuration
├── tsconfig.json          # TypeScript configuration
├── vite.config.ts         # Vite configuration
├── package.json           # Dependencies and scripts
└── Dockerfile             # Docker configuration
```

## Tech Stack

- React 18+
- TypeScript
- TradingView Lightweight Charts
- State Management: TBD (Redux Toolkit/Zustand/Context)
- Styling: TBD (Tailwind CSS/Material-UI/styled-components)
- Build Tool: Vite
- PWA: Workbox

## Development Commands

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Run tests
npm test

# Run linting
npm run lint

# Type checking
npm run type-check
```
