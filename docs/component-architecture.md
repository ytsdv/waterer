# Component Architecture

## Overview

The application has been refactored to use a modular component architecture with individual components extracted from the main page.

## Component Structure

```
src/lib/components/
├── index.ts              # Exports all components and types
├── StatCard.svelte       # Statistics display card
├── SipCard.svelte        # Individual sip record display
├── Button.svelte         # Reusable button component
├── ErrorMessage.svelte   # Error state display
├── LoadingState.svelte   # Loading indicator
├── EmptyState.svelte     # Empty data state
└── SipsList.svelte       # Container for sip records list
```

## Components

### StatCard

- **Purpose**: Display statistics like total sips and total amount
- **Props**: `title: string`, `value: string | number`
- **Features**: Responsive design, dark mode support

### SipCard

- **Purpose**: Display individual sip records
- **Props**: `sip: Sip`
- **Features**: Shows amount, timestamp, notification badge
- **Internal**: Includes `formatDate` function

### Button

- **Purpose**: Reusable button with consistent styling using CVA (class-variance-authority)
- **Props**: `onclick?`, `disabled?`, `type?`, `variant?`, `size?`, `class?`, `children`
- **Variants**:
  - `primary` (default) - Blue background with white text
  - `secondary` - Gray background with dark text
  - `destructive` - Red background for dangerous actions
  - `outline` - Transparent background with border
  - `ghost` - Transparent background, no border
- **Sizes**: `sm`, `md` (default), `lg`
- **Features**: Type-safe variants, hover effects, disabled states, dark mode, extensible styling

### ErrorMessage

- **Purpose**: Display error messages with consistent styling
- **Props**: `message: string`
- **Features**: Red alert styling, dark mode support

### LoadingState

- **Purpose**: Show loading indicators
- **Props**: `message?: string` (defaults to "Loading...")
- **Features**: Centered layout, subtle styling

### EmptyState

- **Purpose**: Display when no data is available
- **Props**: `message?: string` (defaults to "No data available.")
- **Features**: Centered layout, subtle styling

### SipsList

- **Purpose**: Container component for the sips list
- **Props**: `sips: Sip[]`, `loading: boolean`
- **Features**: Combines LoadingState, EmptyState, and SipCard components
- **Layout**: Scrollable list with max height

## Type Definitions

The `Sip` interface is exported from `index.ts` for consistency across components:

```typescript
interface Sip {
  id: number;
  amount: number;
  created_at: string;
  notified_user: boolean;
}
```

## Benefits of This Architecture

1. **Reusability**: Components can be used throughout the application
2. **Maintainability**: Each component has its own styles and logic
3. **Testability**: Components can be tested in isolation
4. **Separation of Concerns**: Each component has a single responsibility
5. **Consistency**: Shared components ensure consistent UI patterns
6. **Type Safety**: TypeScript interfaces ensure proper prop usage
7. **Variant Management**: CVA provides type-safe, extensible component variants
8. **Performance**: Utility-first CSS with efficient class generation
9. **Modern Tailwind**: Uses Tailwind CSS v4 with CSS variables instead of config files

## Main Page Simplification

The main page (`+page.svelte`) is now much cleaner:

- Reduced from ~228 lines to ~45 lines
- Only contains business logic and layout
- All UI components are imported and reused
- Styles are minimal and page-specific only

## Usage Example

```svelte
<script lang="ts">
  import { StatCard, Button, SipsList, type Sip } from "$lib/components";

  let sips: Sip[] = [];
  let loading = false;
</script>

<StatCard title="Total Sips" value={sips.length} />
<Button onclick={handleRefresh}>Refresh</Button>

<!-- Button variants showcase -->
<Button variant="primary" size="lg">Primary Large</Button>
<Button variant="secondary" size="md">Secondary</Button>
<Button variant="destructive" size="sm">Delete</Button>
<Button variant="outline">Outline</Button>
<Button variant="ghost">Ghost</Button>

<SipsList {sips} {loading} />
```

## Recent Enhancements

- ✅ **CVA Integration**: Button component now uses class-variance-authority for type-safe variants
- ✅ **Extended Button Variants**: Added destructive, outline, and ghost variants
- ✅ **Button Sizes**: Added sm, md, lg size variants
- ✅ **Better Type Safety**: CVA provides full TypeScript support for component variants
- ✅ **Tailwind v4 Migration**: Removed config file, now using CSS variables for customization

## Future Enhancements

- Create a Card wrapper component for consistent card styling
- Add animation components for transitions with @tailwindcss/animations
- Create form components (Input, Select, Checkbox, etc.) using CVA
- Add icon components for better visual hierarchy
- Implement Toast/Notification components for user feedback
- Add loading button states and skeleton components
- Create Modal/Dialog components for complex interactions
