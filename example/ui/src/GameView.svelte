<script>
import { createEventDispatcher } from 'svelte';

const dispatch = createEventDispatcher();

export let view;
</script>

<p>{JSON.stringify(view)}</p>

<div>
  <button on:click={() => dispatch('do_action', 'Decr')}>-</button>
  <button on:click={() => dispatch('do_action', 'Incr')}>+</button>
</div>
