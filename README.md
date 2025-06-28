# 💧 Water Tracker

A modern water tracking application built with Tauri, SvelteKit, and TypeScript.

## Features

- **Water Tracking**: Log your daily water intake with detailed history
- **System Notifications**: Get reminders to stay hydrated
- **Modern UI**: Built with Tailwind CSS v4 using CSS variables for customization
- **Component Architecture**: Modular, reusable components with class-variance-authority
- **Cross-Platform**: Desktop app powered by Tauri and Rust
- **Real-time Data**: SQLite database with instant updates

## Tech Stack

- **Frontend**: SvelteKit + TypeScript
- **Styling**: Tailwind CSS v4 (CSS variables approach)
- **Components**: class-variance-authority for type-safe variants
- **Backend**: Tauri + Rust
- **Database**: SQLite with migrations
- **Build Tool**: Vite

## Key Technologies

### Tailwind CSS v4

This project uses the latest Tailwind CSS v4 which replaces JavaScript configuration files with CSS variables:

- ✅ CSS variables in `src/app.css`
- ❌ No `tailwind.config.js`
- 🎨 OKLCH color space for better color consistency
- ⚡ Improved performance and smaller bundles

See [docs/tailwind-v4-migration.md](docs/tailwind-v4-migration.md) for details.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
