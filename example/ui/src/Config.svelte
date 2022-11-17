<script>
import { createEventDispatcher } from 'svelte';

const dispatch = createEventDispatcher();

export let config;
export let readonly;

$: dispatch('config_change', { config: config });
</script>

{#if readonly}
  Config: {JSON.stringify(config)}
{:else}
  <input type="number" bind:value={config}>
{/if}
