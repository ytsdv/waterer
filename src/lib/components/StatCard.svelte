<script lang="ts">
  interface Props {
    title: string;
    value: string | number;
  }

  let { title, value }: Props = $props();
</script>

<div class="bg-white rounded-xl p-6 shadow-lg text-center min-w-0 flex-1 dark:bg-gray-800 dark:text-gray-100">
  <h3 class="m-0 mb-2 text-sm text-gray-600 uppercase tracking-wide dark:text-gray-400">{title}</h3>
  <p class="m-0 text-3xl font-bold text-primary-600 dark:text-primary-400">{value}</p>
</div>
