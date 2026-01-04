# Component Specifications

Detailed technical specifications for all UI components.

## Table of Contents
1. [Button Components](#button-components)
2. [Input Components](#input-components)
3. [Asset Display Components](#asset-display-components)
4. [Chart Components](#chart-components)
5. [Navigation Components](#navigation-components)
6. [Alert Components](#alert-components)
7. [Utility Components](#utility-components)

---

## Button Components

### Primary Button
**Usage:** Main actions (Create, Submit, Save)

```typescript
interface PrimaryButtonProps {
  children: React.ReactNode;
  onClick: () => void;
  disabled?: boolean;
  loading?: boolean;
  size?: 'sm' | 'md' | 'lg';
  fullWidth?: boolean;
}
```

**CSS Specs:**
```css
.btn-primary {
  /* Dimensions */
  height: 40px; /* md size */
  padding: 0 20px;

  /* Colors */
  background: var(--accent-primary);
  color: #FFFFFF;

  /* Typography */
  font-size: 14px;
  font-weight: 500;

  /* Border & Radius */
  border: none;
  border-radius: 6px;

  /* Effects */
  cursor: pointer;
  transition: background 150ms ease;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-primary:active {
  transform: scale(0.98);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Sizes */
.btn-primary--sm { height: 32px; padding: 0 16px; font-size: 13px; }
.btn-primary--lg { height: 48px; padding: 0 24px; font-size: 16px; }
```

**States:**
- Default
- Hover (darker background)
- Active (slight scale down)
- Disabled (50% opacity)
- Loading (spinner + disabled)

---

### Icon Button
**Usage:** Actions with icon only

```typescript
interface IconButtonProps {
  icon: React.ReactNode;
  onClick: () => void;
  ariaLabel: string;
  variant?: 'default' | 'ghost' | 'danger';
  size?: 'sm' | 'md' | 'lg';
}
```

**CSS Specs:**
```css
.btn-icon {
  width: 36px;
  height: 36px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}

.btn-icon:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}
```

---

## Input Components

### Text Input
**Usage:** Form fields, search

```typescript
interface TextInputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  type?: 'text' | 'email' | 'password' | 'number';
  disabled?: boolean;
  error?: string;
  icon?: React.ReactNode;
  onIconClick?: () => void;
}
```

**CSS Specs:**
```css
.input {
  /* Dimensions */
  height: 40px;
  padding: 0 12px;
  width: 100%;

  /* Colors */
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  color: var(--text-primary);

  /* Typography */
  font-size: 14px;
  font-family: inherit;

  /* Border & Radius */
  border-radius: 6px;
  outline: none;

  /* Effects */
  transition: all 150ms ease;
}

.input:focus {
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px rgba(41, 98, 255, 0.1);
}

.input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.input--error {
  border-color: var(--danger);
}
```

---

### Search Input
**Usage:** Asset search, quick search

```typescript
interface SearchInputProps {
  value: string;
  onChange: (value: string) => void;
  onSearch: (query: string) => void;
  placeholder?: string;
  autoFocus?: boolean;
  results?: SearchResult[];
  onResultClick: (result: SearchResult) => void;
}
```

**Features:**
- Debounced search (300ms)
- Clear button when value exists
- Dropdown results on focus
- Keyboard navigation (↑↓ Enter Esc)
- Instant results update

**CSS Specs:**
```css
.search-input-container {
  position: relative;
  width: 100%;
  max-width: 400px;
}

.search-input {
  padding-left: 40px; /* Space for search icon */
  padding-right: 36px; /* Space for clear button */
}

.search-icon {
  position: absolute;
  left: 12px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-tertiary);
  pointer-events: none;
}

.search-clear {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
}

.search-results {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  max-height: 400px;
  overflow-y: auto;
  z-index: 1000;
}

.search-result-item {
  padding: 12px;
  cursor: pointer;
  border-bottom: 1px solid var(--border-primary);
  transition: background 150ms ease;
}

.search-result-item:hover,
.search-result-item--focused {
  background: var(--bg-tertiary);
}
```

---

## Asset Display Components

### Asset Card
**Usage:** Grid/list view of assets

```typescript
interface AssetCardProps {
  symbol: string;
  name: string;
  assetType: 'stock' | 'crypto' | 'etf' | 'bond';
  currentPrice: number;
  change24h: number;
  changePercent24h: number;
  volume24h?: number;
  marketCap?: number;
  onClick: () => void;
}
```

**Layout:**
```
┌─────────────────────────────────────────────────┐
│ AAPL                           $150.25  ▲ +2.5% │
│ Apple Inc. • Stock                              │
│ ─────────────────────────────────────────────── │
│ 24h Vol: 45.2M    Mkt Cap: 2.4T                │
└─────────────────────────────────────────────────┘
```

**CSS Specs:**
```css
.asset-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  padding: 16px;
  cursor: pointer;
  transition: all 150ms ease;
}

.asset-card:hover {
  border-color: var(--accent-primary);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.asset-card__header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 4px;
}

.asset-card__symbol {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.asset-card__price {
  font-family: 'JetBrains Mono', monospace;
  font-size: 18px;
  font-weight: 600;
}

.asset-card__change {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 14px;
  font-weight: 500;
}

.asset-card__change--up { color: var(--success); }
.asset-card__change--down { color: var(--danger); }

.asset-card__name {
  font-size: 14px;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.asset-card__stats {
  display: flex;
  justify-content: space-between;
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--border-primary);
  font-size: 12px;
  color: var(--text-secondary);
}
```

---

### Price Display
**Usage:** Large price displays, real-time updates

```typescript
interface PriceDisplayProps {
  price: number;
  change?: number;
  changePercent?: number;
  currency?: string;
  size?: 'sm' | 'md' | 'lg';
  showChange?: boolean;
  animated?: boolean; // Flash on update
}
```

**CSS Specs:**
```css
.price-display {
  font-family: 'JetBrains Mono', monospace;
  font-weight: 600;
  letter-spacing: -0.02em;
}

.price-display--sm { font-size: 14px; }
.price-display--md { font-size: 18px; }
.price-display--lg { font-size: 32px; }

.price-display--animated-up {
  animation: price-flash-up 500ms ease;
}

.price-display--animated-down {
  animation: price-flash-down 500ms ease;
}
```

---

## Chart Components

### TradingView Chart Container

```typescript
interface ChartContainerProps {
  symbol: string;
  name: string;
  data: ChartData[];
  timeframe: Timeframe;
  onTimeframeChange: (timeframe: Timeframe) => void;
  height?: number;
}

type Timeframe = '1D' | '1W' | '1M' | '3M' | '6M' | '1Y' | '5Y' | 'MAX';
```

**Layout:**
```css
.chart-container {
  background: var(--bg-secondary);
  border-radius: 8px;
  overflow: hidden;
}

.chart-header {
  padding: 16px;
  border-bottom: 1px solid var(--border-primary);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.chart-wrapper {
  height: 600px; /* Desktop */
  position: relative;
}

.chart-timeframe-controls {
  display: flex;
  gap: 8px;
  padding: 16px;
  border-top: 1px solid var(--border-primary);
}

.chart-timeframe-btn {
  padding: 6px 12px;
  font-size: 12px;
  font-weight: 500;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 150ms ease;
}

.chart-timeframe-btn--active {
  background: var(--accent-primary);
  color: #FFFFFF;
}

.chart-timeframe-btn:hover:not(.chart-timeframe-btn--active) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

/* Mobile */
@media (max-width: 768px) {
  .chart-wrapper {
    height: 400px;
  }
}
```

**TradingView Configuration:**
```typescript
const chartOptions = {
  layout: {
    background: { color: 'transparent' },
    textColor: '#787B86',
  },
  grid: {
    vertLines: { color: '#2A2E39' },
    horzLines: { color: '#2A2E39' },
  },
  crosshair: {
    mode: CrosshairMode.Normal,
  },
  timeScale: {
    borderColor: '#2A2E39',
    timeVisible: true,
  },
  rightPriceScale: {
    borderColor: '#2A2E39',
  },
};
```

---

## Navigation Components

### Top Navigation Bar

```typescript
interface NavigationProps {
  user: User | null;
  onLogoClick: () => void;
  onSearch: (query: string) => void;
  notificationCount: number;
}
```

**Layout:**
```css
.navbar {
  height: 56px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
  padding: 0 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  position: sticky;
  top: 0;
  z-index: 100;
}

.navbar__left {
  display: flex;
  align-items: center;
  gap: 32px;
}

.navbar__logo {
  font-size: 20px;
  font-weight: 700;
  color: var(--accent-primary);
  cursor: pointer;
}

.navbar__nav {
  display: flex;
  gap: 24px;
}

.navbar__link {
  color: var(--text-secondary);
  text-decoration: none;
  font-weight: 500;
  transition: color 150ms ease;
}

.navbar__link:hover,
.navbar__link--active {
  color: var(--text-primary);
}

.navbar__right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.navbar__notification-badge {
  position: absolute;
  top: -4px;
  right: -4px;
  background: var(--danger);
  color: white;
  font-size: 10px;
  font-weight: 700;
  min-width: 18px;
  height: 18px;
  border-radius: 9px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 4px;
}
```

---

## Alert Components

### Alert Card

```typescript
interface AlertCardProps {
  id: string;
  symbol: string;
  alertType: 'price' | 'pattern';
  condition: string;
  currentValue: string;
  isActive: boolean;
  onToggle: (id: string) => void;
  onDelete: (id: string) => void;
}
```

**CSS Specs:**
```css
.alert-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  padding: 16px;
  display: flex;
  align-items: center;
  gap: 12px;
}

.alert-card__icon {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
  border-radius: 8px;
  font-size: 20px;
}

.alert-card__content {
  flex: 1;
  min-width: 0;
}

.alert-card__title {
  font-weight: 600;
  font-size: 14px;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.alert-card__description {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.alert-card__status {
  font-size: 12px;
  color: var(--text-tertiary);
}

.alert-card__actions {
  display: flex;
  gap: 8px;
}
```

---

## Utility Components

### Badge

```typescript
interface BadgeProps {
  children: React.ReactNode;
  variant: 'success' | 'danger' | 'warning' | 'info' | 'neutral';
  size?: 'sm' | 'md';
}
```

**CSS Specs:**
```css
.badge {
  display: inline-flex;
  align-items: center;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.badge--sm {
  padding: 2px 6px;
  font-size: 11px;
}

.badge--success {
  background: rgba(38, 166, 154, 0.15);
  color: var(--success);
}

.badge--danger {
  background: rgba(239, 83, 80, 0.15);
  color: var(--danger);
}

.badge--warning {
  background: rgba(255, 152, 0, 0.15);
  color: var(--warning);
}

.badge--info {
  background: rgba(41, 98, 255, 0.15);
  color: var(--info);
}

.badge--neutral {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
}
```

---

### Skeleton Loader

```css
.skeleton {
  background: linear-gradient(
    90deg,
    var(--bg-secondary) 0%,
    var(--bg-tertiary) 50%,
    var(--bg-secondary) 100%
  );
  background-size: 200% 100%;
  animation: skeleton-pulse 2s ease-in-out infinite;
  border-radius: 4px;
}

@keyframes skeleton-pulse {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

/* Common skeleton sizes */
.skeleton--text {
  height: 14px;
  width: 100%;
}

.skeleton--title {
  height: 24px;
  width: 60%;
}

.skeleton--circle {
  border-radius: 50%;
  width: 40px;
  height: 40px;
}

.skeleton--card {
  height: 80px;
  width: 100%;
}
```

---

### Toast Notification

```typescript
interface ToastProps {
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
  duration?: number;
  onClose: () => void;
}
```

**CSS Specs:**
```css
.toast {
  position: fixed;
  bottom: 24px;
  right: 24px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  padding: 16px;
  min-width: 300px;
  max-width: 500px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  display: flex;
  align-items: center;
  gap: 12px;
  animation: toast-slide-in 200ms ease;
  z-index: 9999;
}

@keyframes toast-slide-in {
  from {
    transform: translateX(400px);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

.toast--success { border-left: 4px solid var(--success); }
.toast--error { border-left: 4px solid var(--danger); }
.toast--warning { border-left: 4px solid var(--warning); }
.toast--info { border-left: 4px solid var(--info); }
```

---

**Implementation Priority:**
1. Button, Input, Card (Phase 1)
2. AssetCard, Navigation (Phase 1)
3. Chart, Search (Phase 1)
4. Alert, Toast, Badge (Phase 2)
5. Advanced states and animations (Phase 3)
