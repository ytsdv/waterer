<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    value?: string | number; // Make value optional for backward compatibility
    children?: Snippet;
  }

  let { title, value, children }: Props = $props();
</script>

<div
  class="bg-white rounded-xl p-6 shadow-lg text-center min-w-0 flex-1 dark:bg-gray-800 dark:text-gray-100"
>
  <h3
    class="m-0 mb-2 text-sm text-gray-600 uppercase tracking-wide dark:text-gray-400"
  >
    {title}
  </h3>
  <div class="m-0 text-3xl font-bold text-primary-600 dark:text-primary-400">
    {#if children}
      {@render children()}
    {:else}
      {value}
    {/if}
  </div>
</div>
