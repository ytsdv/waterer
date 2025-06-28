# Water Tracker Component Library

Visual reference for all UI components in the Water Tracker app, with code examples and usage guidelines.

## Usage

This document serves as a practical reference for implementing the components defined in the [Design System](./design-system.md). Each component includes:

- Visual appearance
- HTML structure
- CSS implementation
- Usage examples
- Accessibility considerations

---

## Buttons

### Primary Button

**Use for**: Main actions, form submissions, primary CTAs

```html
<button class="btn-primary">Refresh</button>
```

```css
.btn-primary {
  background-color: #2563eb;
  color: white;
  padding: 0.6em 1.2em;
  border-radius: 8px;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  border: 1px solid transparent;
  transition: all 0.25s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  cursor: pointer;
}

.btn-primary:hover:not(:disabled) {
  background-color: #1d4ed8;
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
```

**States**:

- Default: Blue background (#2563eb)
- Hover: Darker blue (#1d4ed8) + lift effect
- Disabled: 60% opacity
- Loading: Disabled state with loading text

---

## Cards

### Stat Card

**Use for**: Displaying key metrics and statistics

```html
<div class="stat-card">
  <h3>Total Sips</h3>
  <p class="stat-number">42</p>
</div>
```

```css
.stat-card {
  background: white;
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  text-align: center;
  min-width: 150px;
}

.stat-card h3 {
  margin: 0 0 0.5rem 0;
  font-size: 0.9rem;
  color: #666;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.stat-number {
  margin: 0;
  font-size: 2rem;
  font-weight: bold;
  color: #2563eb;
}
```

### Sip Card

**Use for**: Individual sip record display

```html
<div class="sip-card">
  <div class="sip-amount">50ml</div>
  <div class="sip-date">Dec 27, 2024, 2:30 PM</div>
  <div class="notification-badge">✓</div>
</div>
```

```css
.sip-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem;
  background: #f8fafc;
  border-radius: 8px;
  border-left: 4px solid #3b82f6;
  position: relative;
}

.sip-amount {
  font-weight: bold;
  color: #1e40af;
  font-size: 1.1rem;
}

.sip-date {
  color: #64748b;
  font-size: 0.9rem;
}
```

---

## Layout Components

### Container

**Use for**: Main content wrapper with consistent max-width and centering

```html
<main class="container">
  <!-- Content goes here -->
</main>
```

```css
.container {
  margin: 0 auto;
  padding: 2rem;
  max-width: 800px;
}
```

### Stats Section

**Use for**: Horizontal display of key metrics

```html
<div class="stats">
  <div class="stat-card">
    <h3>Total Sips</h3>
    <p class="stat-number">42</p>
  </div>
  <div class="stat-card">
    <h3>Total Amount</h3>
    <p class="stat-number">2100ml</p>
  </div>
</div>
```

```css
.stats {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
  justify-content: center;
}
```

---

## Status Indicators

### Notification Badge

**Use for**: Indicating completion or notification status

```html
<div class="notification-badge">✓</div>
```

```css
.notification-badge {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  background: #10b981;
  color: white;
  border-radius: 50%;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.7rem;
}
```

### Error Message

**Use for**: Displaying error states and messages

```html
<div class="error">Failed to load sips: Network error</div>
```

```css
.error {
  background-color: #fee2e2;
  color: #dc2626;
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 1rem;
  text-align: center;
}
```

### Loading State

**Use for**: Indicating loading content

```html
<div class="loading">Loading sips...</div>
```

```css
.loading {
  text-align: center;
  padding: 2rem;
  color: #666;
  font-style: italic;
}
```

### Empty State

**Use for**: When no content is available

```html
<div class="empty">No sips recorded yet.</div>
```

```css
.empty {
  text-align: center;
  padding: 2rem;
  color: #666;
  font-style: italic;
}
```

---

## List Components

### Sips List

**Use for**: Scrollable list of sip records

```html
<div class="sips-list">
  <div class="sip-card">
    <div class="sip-amount">50ml</div>
    <div class="sip-date">Dec 27, 2024, 2:30 PM</div>
    <div class="notification-badge">✓</div>
  </div>
  <!-- More sip cards... -->
</div>
```

```css
.sips-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  max-height: 400px;
  overflow-y: auto;
}
```

---

## Typography

### Headings

```html
<h1>💧 Water Tracker</h1>
<h2>Previous Sips</h2>
<h3>Total Sips</h3>
```

```css
h1 {
  text-align: center;
  margin-bottom: 2rem;
  font-size: 2.5rem;
}

h2 {
  margin-bottom: 1rem;
  color: #333;
}

h3 {
  margin: 0 0 0.5rem 0;
  font-size: 0.9rem;
  color: #666;
  text-transform: uppercase;
  letter-spacing: 1px;
}
```

---

## Dark Mode Variants

All components automatically adapt to dark mode using CSS media queries:

```css
@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #1a1a1a;
  }

  .stat-card,
  .sips-container {
    background-color: #2a2a2a;
    color: #f6f6f6;
  }

  .sip-card {
    background-color: #3a3a3a;
  }

  h2 {
    color: #f6f6f6;
  }

  .stat-card h3 {
    color: #a1a1a1;
  }

  .sip-date {
    color: #a1a1a1;
  }

  .error {
    background-color: #3f1f1f;
    color: #fca5a5;
  }
}
```

---

## Responsive Behavior

### Mobile Adaptations

```css
@media (max-width: 640px) {
  .stats {
    flex-direction: column;
    align-items: center;
  }

  .stat-card {
    width: 100%;
    max-width: 200px;
  }

  .sip-card {
    flex-direction: column;
    gap: 0.5rem;
    text-align: center;
  }
}
```

**Changes on mobile**:

- Stats cards stack vertically
- Sip cards use centered, stacked layout
- Container padding reduces for better space usage

---

## Accessibility Features

### Focus States

All interactive elements include proper focus indicators:

```css
button:focus,
.focusable:focus {
  outline: 2px solid #2563eb;
  outline-offset: 2px;
}
```

### Semantic HTML

- Use proper heading hierarchy (h1 → h2 → h3)
- Include `alt` attributes for icons when needed
- Use semantic elements (`main`, `section`, `article`)

### Screen Reader Support

- Descriptive text for status indicators
- Proper labeling for interactive elements
- Logical tab order

---

## Common Patterns

### Full Page Layout

```html
<main class="container">
  <h1>💧 Water Tracker</h1>

  <div class="stats">
    <!-- Stat cards -->
  </div>

  <div class="controls">
    <button class="btn-primary">Refresh</button>
  </div>

  <div class="sips-container">
    <h2>Previous Sips</h2>
    <div class="sips-list">
      <!-- Sip cards -->
    </div>
  </div>
</main>
```

### Error Handling

```html
{#if error}
<div class="error">{error}</div>
{/if}
```

### Loading States

```html
{#if loading}
<div class="loading">Loading sips...</div>
{:else if sips.length === 0}
<div class="empty">No sips recorded yet.</div>
{:else}
<!-- Content -->
{/if}
```

---

## Implementation Notes

1. **CSS Custom Properties**: Use CSS variables for consistent theming
2. **Mobile First**: Design and code for mobile, then enhance for desktop
3. **Progressive Enhancement**: Ensure basic functionality works without JavaScript
4. **Performance**: Minimize CSS specificity for better maintainability
5. **Consistency**: Follow the spacing scale and color palette religiously
6. **⚠️ Tailwind v4 Requirement**: Always include `@reference "tailwindcss";` at the top of component style blocks when using `@apply`

---

_This component library should be updated whenever new components are added or existing ones are modified. Keep it in sync with the main design system document._
