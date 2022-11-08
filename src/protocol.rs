use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ids::{PlayerId, ReconnectToken, RoomId, UserId};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    id: UserId,
    username: String,
    leader: bool,
    connected: bool,
    player_id: Option<PlayerId>,
}

// Message from the server to the client.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Error {
        message: String,
    },
    JoinResponse {
        username: String,
        room_id: RoomId,
        token: ReconnectToken,
        user_id: UserId,
    },
    UserInfo {
        users: Vec<UserInfo>,
    },
    RoomInfo {
        config: Value,
    },
    GameInfo {
        view: Value,
    },
    InvalidAction {
        message: String,
    },
}

// Message from the client to the server.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    JoinRoom {
        username: String,
        // None to create a new room.
        room: Option<RoomId>,
    },
    ReconnectRoom {
        token: ReconnectToken,
    },
    UpdateConfig {
        config: Value,
    },
    StartGame {
        player_mapping: Option<HashMap<UserId, Value>>,
    },
    DoAction {
        action: Value,
    },
}
