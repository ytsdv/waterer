import { getContext, setContext } from "svelte";
import { browser } from "$app/environment";

export type Theme = "light" | "dark" | "system";

class ThemeState {
  theme = $state<Theme>("system");

  constructor() {
    this.loadTheme();
    this.applyTheme();
    this.setupSystemThemeListener();
  }

  private loadTheme() {
    if (!browser) return;

    const savedTheme = localStorage.getItem("waterer-theme") as Theme;
    if (savedTheme && ["light", "dark", "system"].includes(savedTheme)) {
      this.theme = savedTheme;
    }
  }

  private saveTheme() {
    if (!browser) return;
    localStorage.setItem("waterer-theme", this.theme);
  }

  private applyTheme() {
    if (!browser) return;

    const root = document.documentElement;

    // Remove existing theme classes
    root.classList.remove("light", "dark");

    if (this.theme === "system") {
      // Let CSS media query handle it
      const prefersDark = window.matchMedia(
        "(prefers-color-scheme: dark)"
      ).matches;
      root.classList.add(prefersDark ? "dark" : "light");
    } else {
      root.classList.add(this.theme);
    }
  }

  setTheme(newTheme: Theme) {
    this.theme = newTheme;
    this.saveTheme();
    this.applyTheme();
  }

  toggleTheme() {
    const themes: Theme[] = ["light", "dark", "system"];
    const currentIndex = themes.indexOf(this.theme);
    const nextIndex = (currentIndex + 1) % themes.length;
    this.setTheme(themes[nextIndex]);
  }

  get isDark(): boolean {
    if (!browser) return false;

    if (this.theme === "system") {
      return window.matchMedia("(prefers-color-scheme: dark)").matches;
    }
    return this.theme === "dark";
  }

  private setupSystemThemeListener() {
    if (!browser) return;

    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const handleChange = () => {
      if (this.theme === "system") {
        this.applyTheme();
      }
    };

    mediaQuery.addEventListener("change", handleChange);
  }

  get themeLabel(): string {
    switch (this.theme) {
      case "light":
        return "Light";
      case "dark":
        return "Dark";
      case "system":
        return "System";
      default:
        return "System";
    }
  }
}

const THEME_STATE_KEY = Symbol("THEME_STATE");

export function setThemeState() {
  return setContext(THEME_STATE_KEY, new ThemeState());
}

export function getThemeState() {
  return getContext<ThemeState>(THEME_STATE_KEY);
}
