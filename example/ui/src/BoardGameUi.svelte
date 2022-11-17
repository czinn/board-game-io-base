<script>
import BoardGameClient from './BoardGameClient.svelte'

export let game_name;

let client;
let username;
let user_id;
let room_id;
let connecting;
let users = [];
let config;
let user;

let new_username;
let rejoin_room_id;

function config_handler(event) {
  client.handle_config_update(event.detail.config);
}
</script>

<main>
  <BoardGameClient
    bind:this={client}
    bind:username={username}
    bind:user_id={user_id}
    bind:room_id={room_id}
    bind:users={users}
    bind:connecting={connecting}
    bind:config={config}
    bind:user={user}
    addr="ws://localhost:9002" />
  <h1>{game_name}</h1>
  {#if connecting}
    <p>Connecting...</p>
  {:else}
    {#if username === null}
      <form on:submit|preventDefault={() => client.join_room(new_username)}>
        <input placeholder="Username" bind:value={new_username}>
        <button type="submit">
          {#if room_id === null}
            Create Room
          {:else}
            Join Room
          {/if}
        </button>
      </form>
      {#if room_id === null}
        <hr/>
        <form on:submit|preventDefault={() => client.rejoin_room(rejoin_room_id) }>
          <input placeholder="Room code" bind:value={rejoin_room_id}>
          <button type="submit">Join Room</button>
        </form>
      {/if}
    {:else}
      <p>
        Players:
        {#each users as u, index}
          {#if index > 0}, {/if}
          <span class="user" class:self={u.id === user_id} class:leader={u.leader}>{u.username}</span>
        {/each}
      </p>
      <p>Room is {room_id}</p>
      <slot name="config" config={config} readonly={!user || !user.leader} config_handler={e => config_handler(e)}></slot>
    {/if}
  {/if}

</main>

<style>
span.user {
}

span.user.self {
  font-weight: bold;
}

span.user.leader {
  color: #ffec00;
}
</style>
