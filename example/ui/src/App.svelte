<script>
import Counter from './lib/Counter.svelte'
import BoardGameClient from './BoardGameClient.svelte'

let client;
let username;
let room_id;

let new_username;
let rejoin_room_id;
</script>

<main>
  <BoardGameClient bind:this={client} bind:username={username} bind:room_id={room_id} addr="ws://localhost:9002" />
  <h1>Example</h1>
  {#if username === null}
    <input bind:value={new_username}>
    <button on:click={() => client.join_room(new_username)}>Join Room</button>
    <br>
    {#if room_id === null}
      <input bind:value={rejoin_room_id}>
      <button on:click={() => client.rejoin_room(rejoin_room_id)}>Rejoin Room</button>
    {/if}
  {:else}
    <p>Your username is {username}</p>
    <p>Room is {room_id}</p>
  {/if}

  <div class="card">
    <Counter />
  </div>

</main>

<style>
</style>
