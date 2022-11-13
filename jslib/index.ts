import type { PlayerId } from "./PlayerId";
import type { ReconnectToken } from "./ReconnectToken";
import type { RoomId } from "./RoomId";
import type { UserId } from "./UserId";
import type { UserInfo } from "./UserInfo";
import type { ServerMessage } from "./ServerMessage";
import type { ClientMessage } from "./ClientMessage";

function get_token(room: RoomId): ReconnectToken | null {
  return window.localStorage.getItem("reconnect_token:" + room);
}

function set_token(room: RoomId, token: ReconnectToken) {
  window.localStorage.setItem("reconnect_token:" + room, token);
}

class Writeable<T> {
  private value: T;
  private subscribers: any[];
  private on_set: ((value: T) => boolean) | null;

  constructor(value: T, on_set?: ((value: T) => boolean)) {
    this.value = value;
    this.subscribers = [];
    this.on_set = on_set || null;
  }

  subscribe(subscription: (value: T) => void): (() => void) {
    subscription(this.value);
    this.subscribers.push(subscription);
    return () => this.subscribers.splice(this.subscribers.indexOf(subscription), 1);
  }

  value_internal(): T {
    return this.value;
  }

  set_internal(value: T) {
    this.value = value;
    for (const subscription of this.subscribers) {
      subscription(value);
    }
  }

  set(value: T) {
    if (this.on_set === null || this.on_set(value)) {
      this.set_internal(value);
    }
  }
}

class Readable<T> {
  private writeable: Writeable<T>;

  constructor(value: T) {
    this.writeable = new Writeable(value);
  }

  subscribe(subscription: (value: T) => void): (() => void) {
    return this.writeable.subscribe(subscription);
  }

  set_internal(value: T) {
    this.writeable.set_internal(value);
  }

  value_internal(): T {
    return this.writeable.value_internal();
  }
}

export class BoardGameClient {
  private ws: WebSocket;
  room_id?: RoomId;
  user_id?: UserId;
  username?: string;
  users: Readable<UserInfo[]>;
  config: Writeable<any>;
  view: Readable<any>;

  constructor(addr: string) {
    this.ws = new WebSocket(addr);
    this.ws.onmessage = event => this.handle_server_message(event);
    this.users = new Readable([]);
    this.config = new Writeable(null, config => this.handle_config_update(config));
    this.view = new Readable(null);
  }

  private handle_config_update(config: any): boolean {
    for (const user of this.users.value_internal()) {
      if (user.id === this.user_id) {
        if (user.leader) {
          this.update_config(config);
          return true;
        } else {
          return false;
        }
      }
    }
    return false;
  }

  private handle_server_message(event: MessageEvent) {
    let data: ServerMessage = JSON.parse(event.data);
    if (data.type === "error") {
      console.log("Error: " + data.message);
    } else if (data.type === "join_response") {
      this.room_id = data.room_id;
      window.history.pushState("", "", "/" + data.room_id);
      set_token(data.room_id, data.token);
      this.user_id = data.user_id;
      this.username = data.username;
    } else if (data.type === "user_info") {
      this.users.set_internal(data.users);
    } else if (data.type === "room_info") {
      this.config.set_internal(data.config);
    } else if (data.type === "game_info") {
      this.view.set_internal(data.view);
    } else if (data.type === "invalid_action") {
      console.log("Invalid action: " + data.message);
    }
  }

  private send_message(data: ClientMessage) {
    this.ws.send(JSON.stringify(data));
  }

  join_room(username: string, room: RoomId | null) {
    this.send_message({ type: "join_room", username, room });
  }

  rejoin_room(room: RoomId): boolean {
    let token: ReconnectToken | null = get_token(room);
    if (token === null) {
      return false;
    }
    this.send_message({ type: "rejoin_room", token, room });
    return true;
  }

  update_config(config: any) {
    this.send_message({ type: "update_config", config });
  }

  start_game(player_mapping?: Record<UserId, PlayerId>) {
    this.send_message({ type: "start_game", player_mapping: player_mapping || null });
  }

  do_action(action: any) {
    this.send_message({ type: "do_action", action });
  }
}
