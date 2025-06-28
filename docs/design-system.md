# Water Tracker Design System

A comprehensive design system for the Water Tracker application, ensuring consistency and maintainability across all UI components.

## Table of Contents

- [Brand Identity](#brand-identity)
- [Color Palette](#color-palette)
- [Typography](#typography)
- [Spacing](#spacing)
- [Components](#components)
- [Layout](#layout)
- [Responsive Design](#responsive-design)
- [Accessibility](#accessibility)
- [Implementation Guidelines](#implementation-guidelines)

## Brand Identity

### Logo & Icon

- **Primary Icon**: 💧 (Water droplet emoji)
- **App Theme**: Clean, minimal, health-focused
- **Personality**: Friendly, encouraging, supportive

### Voice & Tone

- **Encouraging**: Celebrates user progress
- **Simple**: Clear, jargon-free language
- **Supportive**: Non-judgmental tracking approach

## Color Palette

### Primary Colors

```css
/* Primary Blue */
--primary-500: #2563eb;
--primary-600: #1d4ed8;
--primary-700: #1e40af;

/* Success Green */
--success-500: #10b981;
--success-600: #059669;

/* Water Blue Accent */
--water-blue: #3b82f6;
```

### Neutral Colors

```css
/* Light Theme */
--gray-50: #f8fafc;
--gray-100: #f1f5f9;
--gray-200: #e2e8f0;
--gray-300: #cbd5e1;
--gray-400: #94a3b8;
--gray-500: #64748b;
--gray-600: #475569;
--gray-700: #334155;
--gray-800: #1e293b;
--gray-900: #0f172a;

/* Background Colors */
--bg-light: #f6f6f6;
--bg-dark: #1a1a1a;
--surface-light: #ffffff;
--surface-dark: #2a2a2a;
```

### Status Colors

```css
/* Error */
--error-50: #fee2e2;
--error-500: #dc2626;
--error-600: #b91c1c;

/* Warning */
--warning-50: #fef3c7;
--warning-500: #f59e0b;

/* Info */
--info-50: #dbeafe;
--info-500: #3b82f6;
```

### Usage Guidelines

- **Primary Blue**: Main actions, links, active states
- **Success Green**: Completion indicators, positive feedback
- **Water Blue**: Data visualization, progress indicators
- **Grays**: Text, backgrounds, borders
- **Status Colors**: Feedback messages, alerts

## Typography

### Font Family

```css
font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
```

### Font Scale

```css
/* Headers */
--text-4xl: 2.5rem; /* 40px - Main Title */
--text-2xl: 1.5rem; /* 24px - Section Headers */
--text-xl: 1.25rem; /* 20px - Card Titles */
--text-lg: 1.125rem; /* 18px - Emphasized Text */

/* Body */
--text-base: 1rem; /* 16px - Body Text */
--text-sm: 0.875rem; /* 14px - Secondary Text */
--text-xs: 0.75rem; /* 12px - Captions */

/* Stats */
--text-stat: 2rem; /* 32px - Large Numbers */
```

### Font Weights

```css
--font-normal: 400;
--font-medium: 500;
--font-semibold: 600;
--font-bold: 700;
```

### Line Heights

```css
--leading-tight: 1.25;
--leading-normal: 1.5;
--leading-relaxed: 1.625;
```

### Usage Guidelines

- **Headings**: Use semibold (600) for visual hierarchy
- **Body Text**: Regular (400) for readability
- **Emphasis**: Medium (500) for subtle emphasis
- **Stats/Numbers**: Bold (700) for data prominence

## Spacing

### Spacing Scale

```css
--space-1: 0.25rem; /* 4px */
--space-2: 0.5rem; /* 8px */
--space-3: 0.75rem; /* 12px */
--space-4: 1rem; /* 16px */
--space-5: 1.25rem; /* 20px */
--space-6: 1.5rem; /* 24px */
--space-8: 2rem; /* 32px */
--space-12: 3rem; /* 48px */
--space-16: 4rem; /* 64px */
```

### Usage Guidelines

- **Component Padding**: 1rem (16px) minimum
- **Card Padding**: 1.5rem (24px)
- **Section Spacing**: 2rem (32px)
- **Element Gaps**: 0.75rem (12px) for lists, 1rem (16px) for cards

## Components

### Buttons

#### Primary Button

```css
.btn-primary {
  background-color: var(--primary-500);
  color: white;
  padding: 0.6em 1.2em;
  border-radius: 8px;
  font-weight: 500;
  transition: all 0.25s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.btn-primary:hover {
  background-color: var(--primary-600);
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}
```

#### States

- **Default**: Primary blue background
- **Hover**: Darker blue + subtle lift
- **Disabled**: 60% opacity, no pointer events
- **Loading**: Same as disabled with loading text

### Cards

#### Base Card

```css
.card {
  background: white;
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}
```

#### Stat Card

- **Purpose**: Display key metrics
- **Layout**: Centered content with label above value
- **Styling**: Larger text for numbers, muted labels

#### Sip Card

- **Purpose**: Individual sip record display
- **Layout**: Flexbox with amount, date, and badge
- **Styling**: Left border accent, subtle background

### Status Indicators

#### Notification Badge

```css
.notification-badge {
  background: var(--success-500);
  color: white;
  border-radius: 50%;
  width: 20px;
  height: 20px;
  font-size: 0.7rem;
}
```

#### Error Message

```css
.error {
  background-color: var(--error-50);
  color: var(--error-500);
  padding: 1rem;
  border-radius: 8px;
  text-align: center;
}
```

## Layout

### Container

```css
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
}
```

### Grid System

- **Stats Section**: Flexbox with gap for even distribution
- **Sips List**: Vertical flex with consistent gaps
- **Mobile**: Stack horizontally arranged elements

### Scrollable Areas

```css
.scrollable {
  max-height: 400px;
  overflow-y: auto;
}
```

## Responsive Design

### Breakpoints

```css
/* Mobile First Approach */
@media (max-width: 640px) {
  /* Mobile specific styles */
}

@media (min-width: 641px) and (max-width: 1024px) {
  /* Tablet specific styles */
}

@media (min-width: 1025px) {
  /* Desktop specific styles */
}
```

## Accessibility

### Color Contrast

- **Text on Light**: Minimum 4.5:1 contrast ratio
- **Text on Dark**: Sufficient contrast maintained
- **Interactive Elements**: Clear visual distinction

### Focus States

```css
.focusable:focus {
  outline: 2px solid var(--primary-500);
  outline-offset: 2px;
}
```

### Screen Reader Support

- **Semantic HTML**: Use proper heading hierarchy
- **ARIA Labels**: For interactive elements
- **Alt Text**: For decorative elements when needed

## Dark Mode

### Implementation

```css
@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: var(--bg-dark);
  }

  .card {
    background-color: var(--surface-dark);
    color: #f6f6f6;
  }
}
```

### Considerations

- **Automatic Detection**: Respects system preference
- **Color Adjustments**: Maintains contrast ratios
- **Surface Colors**: Appropriate dark variants

## Implementation Guidelines

### Tailwind CSS v4 Integration

This design system is implemented using Tailwind CSS v4 with custom CSS variables for enhanced consistency:

```css
:root {
  --primary-500: #2563eb;
  --primary-600: #1d4ed8;
  --success-500: #10b981;
  --bg-light: #f6f6f6;
  --bg-dark: #1a1a1a;
}
```

### Using Tailwind Classes

Components use Tailwind utility classes combined with custom component classes:

```css
.btn-primary {
  @apply bg-blue-600 text-white px-5 py-3 rounded-lg font-medium 
         transition-all duration-200 shadow-sm cursor-pointer;
}

.stat-card {
  @apply bg-white rounded-xl p-6 shadow-lg text-center min-w-0 flex-1;
}
```

### CSS Custom Properties

CSS custom properties are still used for design tokens that extend beyond Tailwind's defaults:

### Component Structure

1. **Base Styles**: Define core appearance
2. **Variants**: Create modifications for different use cases
3. **States**: Handle hover, active, disabled states
4. **Responsive**: Add mobile-specific adjustments

### Naming Conventions

- **BEM Methodology**: `.block__element--modifier`
- **Semantic Names**: Describe purpose, not appearance
- **Consistent Prefixes**: Use component prefixes when needed

### Performance

- **Efficient Selectors**: Avoid deep nesting
- **Reusable Classes**: Create utility classes for common patterns
- **Minimal Specificity**: Keep specificity low for easier overrides

## Future Considerations

### Extensibility

- **Theme Variables**: Easy to modify for different themes
- **Component Modularity**: Reusable across different views
- **Scalable Architecture**: Supports additional features

### Consistency Checks

- **Regular Audits**: Ensure adherence to design system
- **Documentation Updates**: Keep guidelines current
- **Team Communication**: Share updates with all contributors

---

_This design system is a living document that should evolve with the application. Regular reviews and updates ensure it remains relevant and useful for maintaining design consistency._
