<script lang="ts">
  interface Props {
    label: string;
    type?: "text" | "number" | "email" | "password";
    placeholder?: string;
    value: string | number;
    min?: number;
    max?: number;
    helpText?: string;
    required?: boolean;
    disabled?: boolean;
    id?: string;
  }

  let {
    label,
    type = "text",
    placeholder = "",
    value = $bindable(),
    min,
    max,
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

  <input
    {id}
    {type}
    {placeholder}
    {min}
    {max}
    {required}
    {disabled}
    bind:value
    class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
  />

  {#if helpText}
    <p class="text-xs text-gray-500 dark:text-gray-400">
      {helpText}
    </p>
  {/if}
</div>
