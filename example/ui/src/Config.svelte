<script>
export let config;
export let readonly;
</script>

<p>Readonly: {readonly}</p>

{#if readonly}
  Config: {JSON.stringify(config)}
{:else}
  <input type="number" bind:value={config}>
{/if}
