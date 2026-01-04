# Shared Types

Shared type definitions and interfaces used by both frontend and backend.

## Directory Structure

```
shared/
├── types/
│   ├── user.ts              # User-related types
│   ├── asset.ts             # Asset types (Stock, Crypto, ETF, Bond)
│   ├── watchlist.ts         # Watchlist types
│   ├── alert.ts             # Alert types (Price, Pattern)
│   ├── api.ts               # API request/response types
│   ├── websocket.ts         # WebSocket message types
│   └── index.ts             # Re-export all types
├── package.json             # Package configuration
└── tsconfig.json            # TypeScript configuration
```

## Purpose

This directory contains TypeScript type definitions shared between:
- Frontend (React/TypeScript)
- Backend (if using TypeScript for type generation)

## Usage

### In Frontend
```typescript
import { User, Asset, Watchlist } from '@my-invest/shared';
```

### Type Generation for Rust
Types can be used to generate Rust structs using tools like:
- `ts-rs` (TypeScript to Rust)
- `typeshare` (cross-language type sharing)

## Types Included

- **User**: User account, preferences, authentication
- **Asset**: Stock, Crypto, ETF, Bond data structures
- **Watchlist**: User watchlist configuration
- **Alert**: Price alerts, pattern alerts
- **API**: Request/response DTOs
- **WebSocket**: Real-time message formats
