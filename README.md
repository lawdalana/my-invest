# My-Invest Dashboard

A Progressive Web Application for tracking and analyzing stock market data across multiple asset types (stocks, cryptocurrencies, ETFs, and bonds).

## Project Structure

For detailed structure, see [STRUCTURE.md](./STRUCTURE.md) or [simplified view](./STRUCTURE-SIMPLE.md).

```
my-invest/
├── frontend/                    # React PWA Application
│   ├── src/
│   │   ├── components/         # Reusable UI components
│   │   ├── pages/              # Page components (routes)
│   │   ├── services/           # API clients
│   │   ├── hooks/              # Custom React hooks
│   │   ├── contexts/           # React Context providers
│   │   ├── utils/              # Utility functions
│   │   ├── types/              # TypeScript types
│   │   ├── assets/             # Static assets
│   │   └── styles/             # Global styles
│   ├── public/                 # Public static files
│   ├── tests/                  # Frontend tests
│   └── README.md
│
├── backend/                     # Rust API Server
│   ├── src/
│   │   ├── api/                # API routes and handlers
│   │   ├── services/           # Business logic
│   │   ├── models/             # Data models
│   │   ├── middleware/         # Middleware components
│   │   ├── utils/              # Utility functions
│   │   └── config/             # Configuration
│   ├── tests/                  # Backend tests
│   ├── migrations/             # Database migrations
│   └── README.md
│
├── infrastructure/              # Infrastructure as Code
│   ├── docker/                 # Docker configurations
│   ├── terraform/              # Terraform IaC
│   │   ├── modules/            # Reusable modules
│   │   └── environments/       # Environment configs
│   ├── docker-compose.yml      # Production compose
│   ├── docker-compose.dev.yml  # Development compose
│   └── README.md
│
├── shared/                      # Shared type definitions
│   ├── types/                  # TypeScript types
│   └── README.md
│
├── scripts/                     # Utility scripts
│   ├── dev/                    # Development scripts
│   ├── deploy/                 # Deployment scripts
│   ├── ci/                     # CI scripts
│   └── terraform/              # Terraform scripts
│
├── .github/                     # GitHub configurations
│   └── workflows/              # GitHub Actions workflows
│
├── docs/                        # All documentation
│   ├── REQUIREMENTS.md         # Complete project requirements
│   ├── architecture/           # Architecture & design docs
│   ├── guides/                 # How-to guides
│   ├── api/                    # API documentation
│   ├── development/            # Development docs
│   └── operations/             # Operations & monitoring
│
├── CLAUDE.md                    # AI assistant instructions
├── STRUCTURE.md                 # Detailed folder structure
├── STRUCTURE-SIMPLE.md          # Simplified structure view
├── README.md                    # This file
└── .gitignore                   # Git ignore rules
```

## Tech Stack

### Frontend
- React 18+ with TypeScript
- TradingView Lightweight Charts
- PWA (Progressive Web App)
- State Management: TBD
- Styling: TBD
- Build Tool: Vite

### Backend
- Rust (Actix-web or Axum)
- Tokio async runtime
- JWT authentication
- WebSocket support

### Databases
- MongoDB (user data, watchlists, alerts)
- Redis (cache, real-time data)

### Infrastructure
- Docker & Docker Compose (local development)
- Google Cloud Platform (production)
- Terraform (Infrastructure as Code)
- GitHub Actions (CI/CD)

## Quick Start

### Prerequisites
- Docker and Docker Compose
- Node.js 18+ (for local frontend development)
- Rust 1.70+ (for local backend development)
- Git

### Local Development

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd my-invest
   ```

2. **Start all services with Docker Compose**
   ```bash
   docker-compose -f docker-compose.dev.yml up
   ```

3. **Access the application**
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080
   - MongoDB: localhost:27017
   - Redis: localhost:6379

### Development Without Docker

See individual README files:
- [Frontend Development](./frontend/README.md)
- [Backend Development](./backend/README.md)

## Documentation

- [Requirements](./docs/REQUIREMENTS.md) - Complete project requirements
- [Architecture](./docs/architecture/) - System design and architecture
- [API Documentation](./docs/api/) - API endpoint documentation
- [Deployment Guide](./docs/guides/deployment.md) - Deployment instructions
- [Contributing](./docs/guides/contributing.md) - Contribution guidelines

## Development Workflow

This project follows a git worktree workflow for feature development. See [CLAUDE.md](./CLAUDE.md) for details.

1. Create worktree for feature/bug
2. Implement with senior-dev-architect agent
3. Test with QA engineer agents
4. Review with senior-code-reviewer agent
5. Create pull request
6. Clean up worktree after merge

## Features

- ✅ Multi-asset support (Stocks, Crypto, ETFs, Bonds)
- ✅ Real-time price updates via WebSocket
- ✅ Interactive TradingView charts
- ✅ Customizable watchlists
- ✅ Price alerts (above/below/crosses)
- ✅ Pattern detection alerts
- ✅ Progressive Web App (installable, offline support)
- ✅ Dark/Light theme
- ✅ Responsive design (mobile, tablet, desktop)

## Development Phases

- **Phase 1**: MVP (Auth, basic watchlist, stocks only, simple charts)
- **Phase 2**: Enhanced (Multi-asset, alerts, PWA basics)
- **Phase 3**: Advanced (Patterns, WebSocket, full PWA)
- **Phase 4**: Polish (Dark mode, optimizations)

See [REQUIREMENTS.md](./docs/REQUIREMENTS.md) for detailed phase breakdown.

## License

[License type to be determined]

## Contact

[Contact information to be added]
