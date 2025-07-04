<script lang="ts">
  interface Option {
    value: string;
    label: string;
  }

  interface Props {
    label: string;
    options: Option[];
    value: string;
    helpText?: string;
    required?: boolean;
    disabled?: boolean;
    id?: string;
  }

  let {
    label,
    options,
    value = $bindable(),
    helpText,
    required = false,
    disabled = false,
    id = label.toLowerCase().replace(/\s+/g, "-"),
  }: Props = $props();
</script>

<div class="space-y-2">
  <label
    for={id}
    class="block text-sm font-medium text-gray-700 dark:text-gray-300"
  >
    {label}
    {#if required}
      <span class="text-destructive-500">*</span>
    {/if}
  </label>

  <select
    {id}
    {required}
    {disabled}
    bind:value
    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
  >
    {#each options as option}
      <option value={option.value}>{option.label}</option>
    {/each}
  </select>

  {#if helpText}
    <p class="text-xs text-gray-500 dark:text-gray-400">
      {helpText}
    </p>
  {/if}
</div>
