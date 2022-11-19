<script lang="ts">
import type { PlayerId } from "./bindings/PlayerId";
import type { ReconnectToken } from "./bindings/ReconnectToken";
import type { RoomId } from "./bindings/RoomId";
import type { UserId } from "./bindings/UserId";
import type { UserInfo } from "./bindings/UserInfo";
import type { ServerMessage } from "./bindings/ServerMessage";
import type { ClientMessage } from "./bindings/ClientMessage";

import { onMount } from 'svelte';
import { applyPatch } from 'fast-json-patch'

export type ReconnectData = {
  username: string,
  token: ReconnectToken,
};

function get_tokens(room: RoomId): [ReconnectData] {
  let tokens = JSON.parse(window.localStorage.getItem("reconnect_tokens:" + room));
  if (tokens === null) {
    return [];
  }
  return tokens;
}

function set_tokens(room: RoomId, tokens: [ReconnectData]) {
  window.localStorage.setItem("reconnect_tokens:" + room, JSON.stringify(tokens));
}

function add_token(room: RoomId, username: string, token: ReconnectToken) {
  let tokens = get_tokens(room);
  let found = false;
  for (let i = 0; i < tokens.length; i++) {
    if (tokens[i].token === token) {
      found = true;
      break;
    }
  }
  if (!found) {
    tokens.push({username, token});
    set_tokens(room, tokens);
  }
}

function remove_token(room: RoomId, token: ReconnectToken) {
  let tokens = get_tokens(room);
  for (let i = 0; i < tokens.length; i++) {
    if (tokens[i].token === token) {
      tokens.splice(i, 1);
      set_tokens(room, tokens);
      return;
    }
  }
}

function get_url_room(): RoomId | null {
  if (window.location.pathname.length === 5) {
    // TODO: do more checks here
    return window.location.pathname.slice(1);
  }
  return null;
}

function compute_users_map(users) {
  let users_map = {};
  for (const user of users) {
    users_map[user.id] = user;
  }
  return users_map;
}

// Input properties
export let addr: string;

// Export properties
export let connecting: boolean = true;
export let room_id: RoomId = get_url_room();
export let user_id: UserId = null;
export let username: string = null;
export let users: UserInfo[] = [];
export let config: any = null;
export let view: any = null;
export let reconnect_tokens: [ReconnectData] = [];

// Private properties
let ws: WebSocket;
let server_config: any = null;

// Derived properties
export let users_map: Record<UserId, UserInfo>;
$: users_map = compute_users_map(users);
export let user: UserInfo | null;
$: user = user_id === null ? null : users_map[user_id];

onMount(() => {
  ws = new WebSocket(addr);
  ws.onmessage = event => handle_server_message(event);

  ws.onopen = _ => {
    // Check if the URL has a room code in it
    let url_room = get_url_room();
    if (url_room) {
      room_id = url_room;
      reconnect_tokens = get_tokens(url_room);
    }
    connecting = false;
  };
});

export function handle_config_update(new_config: any) {
  if (new_config !== server_config) {
    if (user !== null && user.leader) {
      update_config(new_config);
    } else {
      config = server_config;
    }
  }
}

function handle_server_message(event: MessageEvent) {
  let data: ServerMessage = JSON.parse(event.data);
  if (data.type === "error") {
    connecting = false;
    console.log("Error: " + data.message);
  } else if (data.type === "join_response") {
    connecting = false;
    user_id = data.user_id;
    username = data.username;
    room_id = data.room_id;
    window.history.pushState("", "", "/" + data.room_id);
    add_token(data.room_id, data.username, data.token);
    reconnect_tokens = get_tokens(data.room_id);
  } else if (data.type === "invalidate_token") {
    connecting = false;
    remove_token(room_id, data.token);
    reconnect_tokens = get_tokens(room_id);
  } else if (data.type === "user_info") {
    users = data.users;
  } else if (data.type === "room_info") {
    server_config = data.config;
    config = server_config;
  } else if (data.type === "game_info") {
    if (data.view !== null) {
      server_config = null;
      config = null;
    }
    view = data.view;
  } else if (data.type === "game_view_diff") {
    if (data.view !== null) {
      try {
        applyPatch(view, data.diff, true);
        view = view;
      } catch (err) {
        send_message({ type: "game_view_request" });
      }
    } else {
      send_message({ type: "game_view_request" });
    }
  } else if (data.type === "invalid_action") {
    console.log("Invalid action: " + data.message);
  }
}

function send_message(data: ClientMessage) {
  ws.send(JSON.stringify(data));
}

export function join_room(username: string) {
  send_message({ type: "join_room", username, room: get_url_room() });
}

export function go_to_room(room: RoomId) {
  window.history.pushState("", "", "/" + room);
  room_id = room;
  reconnect_tokens = get_tokens(room);
}

export function rejoin_room(reconnect_data: ReconnectData) {
  if (room_id === null) {
    return;
  }
  send_message({ type: "rejoin_room", token: reconnect_data.token, room: room_id });
  connecting = true;
}

function update_config(config: any) {
  send_message({ type: "update_config", config });
}

export function start_game() {
  send_message({ type: "start_game" });
}

export function do_action(action: any) {
  send_message({ type: "do_action", action });
}

</script>
