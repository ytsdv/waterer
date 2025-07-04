<script lang="ts">
  import { Sun, Moon, Monitor } from "@lucide/svelte";
  import { getThemeState } from "$lib/ThemeState.svelte";

  const themeState = getThemeState();

  function getThemeIcon(theme: string) {
    switch (theme) {
      case "light":
        return Sun;
      case "dark":
        return Moon;
      case "system":
        return Monitor;
      default:
        return Monitor;
    }
  }

  const Icon = $derived(getThemeIcon(themeState.theme));
</script>

<button
  onclick={() => themeState.toggleTheme()}
  class="flex items-center gap-1 text-xs font-medium text-primary-700 dark:text-primary-100 cursor-pointer hover:underline select-none"
  title="Toggle theme: {themeState.themeLabel}"
>
  <Icon class="w-4 h-4" />
  <span class="hidden sm:inline">{themeState.themeLabel}</span>
</button>
