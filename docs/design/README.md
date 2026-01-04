# Design Documentation

This directory contains all UI/UX design documentation for the My-Invest Dashboard.

## Files

### [ui-design.md](../ui-design.md)
Complete UI/UX design system including:
- Design principles and philosophy
- Color palette (dark and light themes)
- Typography system
- Spacing and layout
- Page layouts and wireframes
- Performance optimizations
- Accessibility guidelines

### [component-specifications.md](./component-specifications.md)
Detailed technical specifications for all UI components:
- Button components (Primary, Secondary, Icon)
- Input components (Text, Search)
- Asset display components (Card, Price Display)
- Chart components (TradingView integration)
- Navigation components
- Alert components
- Utility components (Badge, Skeleton, Toast)

### mockups/
Directory for design mockups and screenshots:
- Figma exports
- Prototype screenshots
- User flow diagrams

## Design Principles

1. **Performance First** - Every design decision considers performance
2. **Minimal & Clean** - Remove visual clutter, focus on data
3. **TradingView-inspired** - Professional financial application aesthetic
4. **Accessible** - WCAG 2.1 AA compliant
5. **Responsive** - Mobile-first approach

## Quick Reference

### Colors
- **Primary:** `#2962FF` (Blue)
- **Success:** `#26A69A` (Teal/Green)
- **Danger:** `#EF5350` (Red)
- **Background (Dark):** `#131722`

### Typography
- **Font:** Inter (UI), JetBrains Mono (Prices)
- **Base Size:** 14px
- **Scale:** 11px, 12px, 14px, 16px, 20px, 24px, 32px

### Spacing
- **Base Unit:** 4px
- **Scale:** 4px, 8px, 12px, 16px, 20px, 24px, 32px, 40px, 48px

### Breakpoints
- **Mobile:** < 640px
- **Tablet:** 640px - 1024px
- **Desktop:** > 1024px

## Implementation

### CSS Variables
All design tokens are defined as CSS custom properties in:
```
/frontend/src/styles/theme.ts
```

### Component Library
React components are implemented in:
```
/frontend/src/components/
```

### Icons
Using Lucide React icons:
```bash
npm install lucide-react
```

## Design Tools

- **Prototyping:** Figma (future)
- **Icons:** Lucide React
- **Charts:** TradingView Lightweight Charts
- **Component Dev:** Storybook
- **Testing:** Chromatic (visual regression)

## Performance Targets

- **FCP:** < 1.5s
- **LCP:** < 2.5s
- **TTI:** < 3.5s
- **CLS:** < 0.1
- **Chart Render:** < 500ms

## Accessibility

All components must meet:
- WCAG 2.1 Level AA
- Keyboard navigation support
- Screen reader compatibility
- Minimum contrast ratios (4.5:1)

## Contributing

When adding new components or modifying designs:

1. Update design documentation first
2. Ensure consistency with design system
3. Consider performance implications
4. Test accessibility
5. Document in Storybook
6. Update this README if needed

## Resources

- [TradingView Charting Library](https://www.tradingview.com/HTML5-stock-forex-bitcoin-charting-library/)
- [Lucide Icons](https://lucide.dev/)
- [WCAG Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [Web Vitals](https://web.dev/vitals/)
