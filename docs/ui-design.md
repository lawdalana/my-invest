# UI/UX Design - My-Invest Dashboard

**Design Philosophy:** Minimal, Modern, High-Performance, TradingView-inspired

## Design Principles

1. **Performance First** - Optimize for speed and responsiveness
2. **Data Density** - Maximize information display without clutter
3. **Clean & Minimal** - Remove unnecessary visual elements
4. **Professional** - Financial-grade interface aesthetics
5. **Accessible** - WCAG 2.1 AA compliant
6. **Responsive** - Seamless mobile to desktop experience

## Color Palette

### Dark Theme (Default)
```css
/* Primary Colors */
--bg-primary: #131722;        /* Main background (TradingView dark) */
--bg-secondary: #1E222D;      /* Cards, panels */
--bg-tertiary: #2A2E39;       /* Elevated surfaces */

/* Text Colors */
--text-primary: #D1D4DC;      /* Primary text */
--text-secondary: #787B86;    /* Secondary text */
--text-tertiary: #434651;     /* Disabled text */

/* Accent Colors */
--accent-primary: #2962FF;    /* Primary actions, links */
--accent-hover: #1E53E5;      /* Hover states */

/* Status Colors */
--success: #26A69A;           /* Positive/Up */
--danger: #EF5350;            /* Negative/Down */
--warning: #FF9800;           /* Alerts */
--info: #2962FF;              /* Information */

/* Chart Colors */
--chart-up: #26A69A;          /* Bullish candles */
--chart-down: #EF5350;        /* Bearish candles */
--chart-grid: #2A2E39;        /* Grid lines */
--chart-text: #787B86;        /* Chart labels */

/* Border Colors */
--border-primary: #2A2E39;
--border-secondary: #434651;
```

### Light Theme
```css
/* Primary Colors */
--bg-primary: #FFFFFF;
--bg-secondary: #F7F9FB;
--bg-tertiary: #E0E3EB;

/* Text Colors */
--text-primary: #131722;
--text-secondary: #434651;
--text-tertiary: #787B86;

/* Accent Colors */
--accent-primary: #2962FF;
--accent-hover: #1E53E5;

/* Status Colors */
--success: #00897B;
--danger: #D84315;
--warning: #F57C00;
--info: #1976D2;

/* Chart Colors */
--chart-up: #00897B;
--chart-down: #D84315;
--chart-grid: #E0E3EB;
--chart-text: #787B86;

/* Border Colors */
--border-primary: #E0E3EB;
--border-secondary: #B2B5BE;
```

## Typography

### Font Family
- **Primary:** 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif
- **Monospace:** 'JetBrains Mono', 'Roboto Mono', monospace (for prices, numbers)

### Font Sizes
```css
--text-xs: 11px;     /* Labels, footnotes */
--text-sm: 12px;     /* Secondary text, table data */
--text-base: 14px;   /* Body text, primary content */
--text-lg: 16px;     /* Headings, emphasis */
--text-xl: 20px;     /* Section headers */
--text-2xl: 24px;    /* Page titles */
--text-3xl: 32px;    /* Large displays */
--text-price: 18px;  /* Price displays (monospace) */
```

### Font Weights
```css
--font-normal: 400;
--font-medium: 500;
--font-semibold: 600;
--font-bold: 700;
```

## Spacing System

```css
--space-1: 4px;
--space-2: 8px;
--space-3: 12px;
--space-4: 16px;
--space-5: 20px;
--space-6: 24px;
--space-8: 32px;
--space-10: 40px;
--space-12: 48px;
```

## Component Design

### 1. Navigation Bar (Top)
```
┌─────────────────────────────────────────────────────────┐
│ [Logo] Dashboard  Watchlists  Alerts  [Search] [🔔] [👤] │
└─────────────────────────────────────────────────────────┘
```
- **Height:** 56px
- **Background:** `--bg-secondary`
- **Border Bottom:** 1px solid `--border-primary`
- **Fixed position** for scroll persistence
- **Search bar** with instant results (max-width: 400px)
- **Icons** for notifications and user profile

### 2. Asset Card (List View)
```
┌─────────────────────────────────────────────────┐
│ AAPL                           $150.25  ▲ +2.5% │
│ Apple Inc. • Stock                              │
│ ─────────────────────────────────────────────── │
│ 24h Vol: 45.2M    Mkt Cap: 2.4T                │
└─────────────────────────────────────────────────┘
```
- **Height:** 80px
- **Padding:** 16px
- **Background:** `--bg-secondary`
- **Border Radius:** 8px
- **Hover:** Slight elevation + border highlight
- **Click:** Navigate to detail view

### 3. Price Display
```css
/* Large Price Display */
.price {
  font-family: 'JetBrains Mono', monospace;
  font-size: var(--text-price);
  font-weight: 600;
  letter-spacing: -0.02em;
}

/* Price Change */
.price-change-up {
  color: var(--success);
}
.price-change-down {
  color: var(--danger);
}
```

### 4. Chart Container
```
┌─────────────────────────────────────────────────┐
│ AAPL - Apple Inc.                     $150.25   │
│ ───────────────────────────────────────────────│
│                                                 │
│                 [Chart Area]                    │
│                 600px height                    │
│                                                 │
│ ───────────────────────────────────────────────│
│ [1D] [1W] [1M] [3M] [6M] [1Y] [5Y] [MAX]      │
└─────────────────────────────────────────────────┘
```
- **Default Height:** 600px (desktop), 400px (mobile)
- **TradingView Lightweight Charts** integration
- **Timeframe buttons** at bottom
- **Minimal UI** - chart fills entire container
- **Grid:** Subtle, non-distracting

### 5. Watchlist Tabs
```
┌─────────────────────────────────────────┐
│ [My Stocks] [Crypto] [ETFs] [+ New]    │
└─────────────────────────────────────────┘
```
- **Style:** Minimal tabs with bottom border indicator
- **Active:** `--accent-primary` bottom border (3px)
- **Hover:** `--text-primary` color
- **Inactive:** `--text-secondary` color

### 6. Alert Card
```
┌─────────────────────────────────────────┐
│ ⚠️ AAPL Price Alert          [🔔] [✕]  │
│ Notify when price crosses $155.00       │
│ Current: $150.25 | Active               │
└─────────────────────────────────────────┘
```
- **Compact design** (60px height)
- **Toggle switch** for enable/disable
- **Delete button** on hover

### 7. Search Results (Dropdown)
```
┌────────────────────────────────────────┐
│ AAPL - Apple Inc.            $150.25 ▲ │
│ Stock                                  │
│ ────────────────────────────────────── │
│ GOOGL - Alphabet Inc.        $140.30 ▼ │
│ Stock                                  │
│ ────────────────────────────────────── │
│ BTC-USD - Bitcoin            $45,000 ▲ │
│ Cryptocurrency                         │
└────────────────────────────────────────┘
```
- **Max Height:** 400px (scrollable)
- **Instant search** - no delay
- **Keyboard navigation** support
- **Fuzzy matching** for symbols and names

### 8. Filter Chips
```
[All] [Stocks] [Crypto] [ETFs] [Bonds]
```
- **Height:** 32px
- **Border Radius:** 16px
- **Active:** `--accent-primary` background
- **Inactive:** `--bg-tertiary` background
- **Transition:** 150ms ease

### 9. Button Styles

#### Primary Button
```css
.btn-primary {
  background: var(--accent-primary);
  color: #FFFFFF;
  padding: 10px 20px;
  border-radius: 6px;
  font-weight: 500;
  transition: background 150ms ease;
}
.btn-primary:hover {
  background: var(--accent-hover);
}
```

#### Secondary Button
```css
.btn-secondary {
  background: transparent;
  border: 1px solid var(--border-secondary);
  color: var(--text-primary);
  padding: 10px 20px;
  border-radius: 6px;
}
```

#### Icon Button
```css
.btn-icon {
  width: 36px;
  height: 36px;
  border-radius: 6px;
  background: transparent;
  display: flex;
  align-items: center;
  justify-content: center;
}
.btn-icon:hover {
  background: var(--bg-tertiary);
}
```

## Page Layouts

### Dashboard (Main View)
```
┌──────────────────────────────────────────────────────┐
│ Navigation Bar                                       │
├──────────────────────────────────────────────────────┤
│                                                      │
│ ┌──────────────┬─────────────────────────────────┐ │
│ │ Filters      │ [All] [Stocks] [Crypto] [ETFs] │ │
│ └──────────────┴─────────────────────────────────┘ │
│                                                      │
│ ┌────────────────────────────────────────────────┐ │
│ │ Asset List (Grid/Table)                        │ │
│ │ ┌─────────────┐ ┌─────────────┐ ┌───────────┐ │ │
│ │ │ Asset Card  │ │ Asset Card  │ │ Card      │ │ │
│ │ └─────────────┘ └─────────────┘ └───────────┘ │ │
│ │ ┌─────────────┐ ┌─────────────┐ ┌───────────┐ │ │
│ │ │ Asset Card  │ │ Asset Card  │ │ Card      │ │ │
│ │ └─────────────┘ └─────────────┘ └───────────┘ │ │
│ └────────────────────────────────────────────────┘ │
│                                                      │
└──────────────────────────────────────────────────────┘
```

**Layout:**
- **Container:** Max-width 1440px, centered
- **Padding:** 24px (desktop), 16px (mobile)
- **Grid:** 3 columns (desktop), 2 columns (tablet), 1 column (mobile)
- **Gap:** 16px

### Asset Detail Page
```
┌──────────────────────────────────────────────────────┐
│ Navigation Bar                                       │
├──────────────────────────────────────────────────────┤
│ ← Back                                               │
│                                                      │
│ ┌────────────────────────────────────────────────┐ │
│ │ AAPL - Apple Inc.                  $150.25 ▲   │ │
│ │ +$3.75 (+2.57%) • Today                        │ │
│ └────────────────────────────────────────────────┘ │
│                                                      │
│ ┌────────────────────────────────────────────────┐ │
│ │                                                │ │
│ │           TradingView Chart                    │ │
│ │           (600px height)                       │ │
│ │                                                │ │
│ │ [1D] [1W] [1M] [3M] [6M] [1Y] [5Y] [MAX]      │ │
│ └────────────────────────────────────────────────┘ │
│                                                      │
│ ┌──────────────┐ ┌──────────────┐                  │
│ │ + Watchlist  │ │ + Alert      │                  │
│ └──────────────┘ └──────────────┘                  │
│                                                      │
│ ┌────────────────────────────────────────────────┐ │
│ │ Market Stats                                   │ │
│ │ Open: $147.50  High: $151.20  Low: $146.80    │ │
│ │ Volume: 45.2M  Market Cap: $2.4T               │ │
│ └────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘
```

### Watchlist Management Page
```
┌──────────────────────────────────────────────────────┐
│ Navigation Bar                                       │
├──────────────────────────────────────────────────────┤
│ My Watchlists                          [+ New]       │
│                                                      │
│ ┌────────────────────────────────────────────────┐ │
│ │ [Tech Stocks ▾] [Edit] [Delete]               │ │
│ │ ────────────────────────────────────────────── │ │
│ │ AAPL - Apple Inc.              $150.25 ▲ +2.5% │ │
│ │ GOOGL - Alphabet Inc.          $140.30 ▲ +1.2% │ │
│ │ MSFT - Microsoft Corp.         $380.50 ▼ -0.5% │ │
│ └────────────────────────────────────────────────┘ │
│                                                      │
│ ┌────────────────────────────────────────────────┐ │
│ │ [Crypto ▾] [Edit] [Delete]                     │ │
│ │ ────────────────────────────────────────────── │ │
│ │ BTC-USD - Bitcoin              $45,000 ▲ +3.2% │ │
│ │ ETH-USD - Ethereum             $2,500 ▲ +2.1%  │ │
│ └────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘
```

### Alerts Management Page
```
┌──────────────────────────────────────────────────────┐
│ Navigation Bar                                       │
├──────────────────────────────────────────────────────┤
│ My Alerts                              [+ New Alert] │
│                                                      │
│ [Active] [Triggered] [All]                          │
│                                                      │
│ ┌────────────────────────────────────────────────┐ │
│ │ ⚠️ AAPL Price Alert          [🔔 Active] [✕]  │ │
│ │ Notify when price crosses $155.00              │ │
│ │ Current: $150.25                               │ │
│ └────────────────────────────────────────────────┘ │
│                                                      │
│ ┌────────────────────────────────────────────────┐ │
│ │ 📊 BTC Pattern Alert         [🔔 Active] [✕]  │ │
│ │ Golden Cross detected                          │ │
│ │ RSI: 68.5 (Approaching overbought)             │ │
│ └────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────┘
```

## Responsive Breakpoints

```css
/* Mobile First */
--breakpoint-sm: 640px;   /* Small devices */
--breakpoint-md: 768px;   /* Tablets */
--breakpoint-lg: 1024px;  /* Laptops */
--breakpoint-xl: 1280px;  /* Desktops */
--breakpoint-2xl: 1536px; /* Large desktops */
```

### Mobile Optimizations (< 768px)
- **Navigation:** Hamburger menu
- **Asset Cards:** Full width, single column
- **Chart Height:** 400px (reduced)
- **Font Sizes:** Slightly smaller
- **Padding:** Reduced to 16px
- **Tabs:** Horizontal scroll if needed

### Tablet (768px - 1024px)
- **Asset Grid:** 2 columns
- **Chart Height:** 500px
- **Side panels:** Collapsible

### Desktop (> 1024px)
- **Asset Grid:** 3 columns
- **Chart Height:** 600px
- **Full navigation visible**

## Performance Optimizations

### 1. Virtual Scrolling
```javascript
// For large asset lists (100+ items)
// Only render visible items + buffer
// Use react-window or react-virtualized
```

### 2. Chart Performance
- **TradingView Lightweight Charts** (60 FPS)
- Render only visible data points
- Throttle real-time updates (max 30 FPS)
- Use canvas rendering (not SVG)

### 3. Image Optimization
- Use WebP format with fallbacks
- Lazy load images below the fold
- Use responsive image sizes

### 4. Code Splitting
```javascript
// Lazy load routes
const Dashboard = lazy(() => import('./pages/Dashboard'));
const AssetDetail = lazy(() => import('./pages/AssetDetail'));
```

### 5. Debouncing & Throttling
```javascript
// Search input: 300ms debounce
// Price updates: 1000ms throttle
// Scroll events: 100ms throttle
```

### 6. CSS Optimizations
- Use CSS transforms for animations (GPU accelerated)
- Minimize repaints with `will-change`
- Use CSS Grid and Flexbox (not floats)

### 7. Data Caching
- Cache API responses (5 minutes for prices)
- Use IndexedDB for offline data
- Implement optimistic UI updates

## Animations & Transitions

### Micro-interactions
```css
/* Smooth transitions */
.transition-default {
  transition: all 150ms cubic-bezier(0.4, 0, 0.2, 1);
}

/* Price change flash */
@keyframes price-flash-up {
  0% { background-color: transparent; }
  50% { background-color: rgba(38, 166, 154, 0.2); }
  100% { background-color: transparent; }
}

@keyframes price-flash-down {
  0% { background-color: transparent; }
  50% { background-color: rgba(239, 83, 80, 0.2); }
  100% { background-color: transparent; }
}
```

### Loading States
```
┌─────────────────────────────────┐
│ ░░░░░░░░░░░░░░░░░  Skeleton    │
│ ░░░░░░░░░░░░       Loading     │
└─────────────────────────────────┘
```
- Use skeleton screens (no spinners)
- Pulse animation: `animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;`

## Accessibility (WCAG 2.1 AA)

### Contrast Ratios
- **Normal text:** Minimum 4.5:1
- **Large text:** Minimum 3:1
- **UI components:** Minimum 3:1

### Keyboard Navigation
- All interactive elements focusable
- Visible focus indicators
- Logical tab order
- Keyboard shortcuts for power users

### Screen Readers
- Proper ARIA labels
- Semantic HTML
- Alt text for images
- Live regions for price updates

### Focus States
```css
:focus-visible {
  outline: 2px solid var(--accent-primary);
  outline-offset: 2px;
}
```

## Icons

**Library:** Lucide React (lightweight, modern)

Common icons:
- `TrendingUp`, `TrendingDown` - Price changes
- `Bell` - Notifications
- `Search` - Search functionality
- `Plus` - Add actions
- `X` - Close/delete
- `Settings` - Configuration
- `User` - Profile
- `BarChart3` - Charts
- `Eye` - Watchlist

## Dark/Light Mode Toggle

```
┌──────────────────┐
│ ☀️ [Toggle] 🌙  │
└──────────────────┘
```

**Position:** Top right in navigation
**Style:** Smooth transition (200ms)
**Storage:** LocalStorage persistence

## Component Library Summary

### Base Components
1. Button (Primary, Secondary, Icon, Link)
2. Input (Text, Search, Number)
3. Card (Container for content)
4. Badge (Status indicators)
5. Chip (Filter tags)
6. Toggle (Switch)
7. Dropdown (Select, Menu)
8. Modal (Dialogs)
9. Toast (Notifications)
10. Skeleton (Loading states)

### Complex Components
1. AssetCard - Display asset summary
2. Chart - TradingView integration
3. SearchBar - Autocomplete search
4. WatchlistTabs - Tab navigation
5. AlertCard - Alert management
6. PriceDisplay - Real-time price
7. Navigation - Top nav bar
8. FilterBar - Asset filtering

## Performance Targets

- **First Contentful Paint (FCP):** < 1.5s
- **Largest Contentful Paint (LCP):** < 2.5s
- **Time to Interactive (TTI):** < 3.5s
- **Cumulative Layout Shift (CLS):** < 0.1
- **Chart Render Time:** < 500ms
- **Search Response Time:** < 300ms
- **Price Update Latency:** < 100ms

## Design System Tools

### Development
- **Storybook** - Component development
- **Figma** - Design mockups (future)
- **Tailwind CSS** (optional) - Utility-first CSS

### Testing
- **Chromatic** - Visual regression testing
- **Lighthouse** - Performance audits
- **axe DevTools** - Accessibility testing

## Implementation Priority

### Phase 1 (MVP)
1. Dark theme only
2. Core components (Button, Input, Card)
3. Navigation bar
4. Asset card (list view)
5. Basic chart integration
6. Search functionality

### Phase 2
1. Light theme
2. Advanced filtering
3. Watchlist management UI
4. Alert creation UI
5. Responsive optimizations

### Phase 3
1. Virtual scrolling
2. Advanced animations
3. PWA optimizations
4. Offline UI states
5. Accessibility refinements

## Design Files Location

- **Mockups:** `/docs/design/mockups/` (Figma exports)
- **Assets:** `/frontend/src/assets/` (Icons, logos)
- **Design Tokens:** `/frontend/src/styles/theme.ts`
- **Component Library:** `/frontend/src/components/`

---

**Note:** This design prioritizes performance and usability. Every visual element serves a functional purpose. The TradingView-inspired aesthetic ensures familiarity for financial application users while maintaining modern design standards.
