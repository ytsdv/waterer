# Tailwind CSS v4 Usage Guide

This document explains how to use Tailwind CSS v4 in the Water Tracker project.

## Setup Complete ✅

Tailwind CSS v4 has been successfully integrated into the project with:

- ✅ Tailwind CSS v4 installed
- ✅ PostCSS configuration
- ✅ Custom design system variables
- ✅ Component-scoped styles using `@apply`
- ✅ Clean global CSS with only variables and base styles
- ✅ Design system documentation updated

## Architecture

The project now follows a **component-scoped styling approach**:

- **Global CSS** (`src/app.css`): Only CSS variables and base theme settings
- **Component Styles**: All `@apply` statements moved to individual Svelte components
- **Benefits**: Better organization, easier maintenance, component isolation

## File Structure

```
src/
├── app.css                 # Global CSS variables and base styles
├── routes/
│   ├── +layout.svelte     # Global CSS import
│   └── +page.svelte       # Main page with component-specific styles
├── docs/
│   ├── design-system.md   # Updated with Tailwind info
│   └── component-library.md
postcss.config.js          # PostCSS configuration
tailwind.config.js          # Tailwind configuration
```

## How to Use

### 1. Existing Component Classes

Use the predefined component classes that combine Tailwind utilities:

```html
<!-- Primary button -->
<button class="btn-primary">Click me</button>

<!-- Stat card -->
<div class="stat-card">
  <h3>Total Sips</h3>
  <p class="stat-number">42</p>
</div>

<!-- Sip record -->
<div class="sip-card">
  <div class="sip-amount">50ml</div>
  <div class="sip-date">Dec 27, 2024</div>
</div>
```

### 2. Pure Tailwind Utilities

You can also use Tailwind utilities directly:

```html
<!-- Using pure Tailwind classes -->
<div class="bg-blue-600 text-white p-4 rounded-lg shadow-md">
  <h2 class="text-xl font-bold mb-2">Custom Card</h2>
  <p class="text-blue-100">Using Tailwind utilities</p>
</div>
```

### 3. Custom Colors

The project includes custom color tokens:

```html
<!-- Using custom colors from design system -->
<div class="bg-primary-500 text-white">Primary color</div>
<div class="bg-success-500 text-white">Success color</div>
<div class="border-l-4 border-water-blue">Water theme</div>
```

## Available Component Classes

### Buttons

- `.btn-primary` - Main action button with hover effects

### Cards

- `.stat-card` - Statistics display card
- `.sip-card` - Individual sip record card
- `.sips-container` - Container for sip lists

### Layout

- `.container` - Main content wrapper
- `.stats` - Flexbox stats section
- `.controls` - Centered control section
- `.sips-list` - Scrollable sip list

### Status Indicators

- `.notification-badge` - Success/completion badge
- `.error` - Error message styling
- `.loading` / `.empty` - State indicators

## Custom CSS Variables

The following CSS variables are available for consistency:

```css
/* Colors */
--primary-500: #2563eb
--primary-600: #1d4ed8
--success-500: #10b981
--water-blue: #3b82f6

/* Typography */
--text-4xl: 2.5rem
--text-stat: 2rem

/* Spacing */
--space-4: 1rem
--space-8: 2rem
```

## Dark Mode

Dark mode is handled automatically through:

1. **CSS Media Queries**: `@media (prefers-color-scheme: dark)`
2. **Component Classes**: Automatically adapt to dark mode
3. **Tailwind Dark Utilities**: Use `dark:` prefix when needed

```html
<!-- Tailwind dark mode utilities -->
<div class="bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100">
  Adapts to system theme
</div>
```

## Development Workflow

### 1. Starting Development

```bash
pnpm tauri dev
```

### 2. Building for Production

```bash
pnpm tauri build
```

### 3. Adding New Styles

**Option A: Use existing component classes**

```html
<div class="stat-card">...</div>
```

**Option B: Create new component class in your Svelte component**

```svelte
<style>
  @reference "tailwindcss";

  .new-component {
    @apply bg-white rounded-lg p-4 shadow-md;
  }
</style>
```

**Option C: Use Tailwind utilities directly**

```html
<div class="bg-white rounded-lg p-4 shadow-md">...</div>
```

## Best Practices

### 1. Prefer Component Classes

Use predefined component classes for consistency:

```html
<!-- ✅ Good -->
<button class="btn-primary">Submit</button>

<!-- ❌ Avoid -->
<button class="bg-blue-600 text-white px-5 py-3 rounded-lg">Submit</button>
```

### 2. Follow Design System

Stick to the design system colors and spacing:

```html
<!-- ✅ Good - uses design system spacing -->
<div class="p-6 mb-8">
  <!-- ❌ Avoid - arbitrary values -->
  <div class="p-[17px] mb-[33px]"></div>
</div>
```

### 3. Mobile-First Responsive

Design for mobile first, then enhance:

```html
<!-- ✅ Good - mobile-first responsive -->
<div class="flex-col md:flex-row">
  <!-- ❌ Avoid - desktop-first -->
  <div class="flex-row md:flex-col"></div>
</div>
```

## Important: Tailwind CSS v4 Requirements

### Using @apply in Components

**⚠️ Required:** When using `@apply` directives in Svelte component `<style>` blocks, you MUST include the `@reference` directive:

```svelte
<style>
  @reference "tailwindcss";

  .your-class {
    @apply bg-blue-600 text-white px-4 py-2;
  }
</style>
```

**Why?** Tailwind CSS v4 needs this directive to make utility classes available for `@apply` in component-scoped styles.

**Error without @reference:**

```
Cannot apply unknown utility class `px-8`. Are you using CSS modules or similar and missing `@reference`?
```

## Troubleshooting

### Styles Not Applying

1. Check if CSS is imported in `+layout.svelte`
2. Verify PostCSS is processing the file
3. Check Tailwind configuration includes your files
4. **Ensure `@reference "tailwindcss";` is at the top of component style blocks**

### Build Issues

1. Ensure all dependencies are installed: `pnpm install`
2. Clear cache: `rm -rf node_modules/.cache`
3. Restart development server

### Custom Classes Not Working

1. Check syntax in your component's `<style>` block
2. Ensure `@apply` directives are valid
3. Verify Tailwind classes exist
4. Restart development server after adding new component styles

## Resources

- [Tailwind CSS v4 Documentation](https://tailwindcss.com/docs)
- [Design System Documentation](./design-system.md)
- [Component Library](./component-library.md)
- [SvelteKit Documentation](https://kit.svelte.dev/)

---

_This setup provides a scalable, maintainable styling solution that combines the power of Tailwind CSS with a consistent design system._
