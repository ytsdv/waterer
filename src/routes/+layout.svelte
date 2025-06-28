<script lang="ts">
  import { setAppState } from '$lib/AppState.svelte';
  import '../app.css';
  
  let { children } = $props();

  setAppState();
</script>

{@render children()}
