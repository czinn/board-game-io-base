use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ids::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserInfo {
    id: UserId,
    username: String,
    leader: bool,
    connected: bool,
    player_id: Option<PlayerId>,
}

impl UserInfo {
    pub fn new(
        id: UserId,
        username: String,
        leader: bool,
        connected: bool,
        player_id: Option<PlayerId>,
    ) -> Self {
        Self {
            id,
            username,
            leader,
            connected,
            player_id,
        }
    }
}

// Message from the server to the client.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Error {
        message: String,
    },
    JoinResponse {
        room_id: RoomId,
        token: ReconnectToken,
        user_id: UserId,
        username: String,
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
    RejoinRoom {
        token: ReconnectToken,
        room: RoomId,
    },
    UpdateConfig {
        config: Value,
    },
    StartGame {
        player_mapping: Option<HashMap<UserId, PlayerId>>,
    },
    DoAction {
        action: Value,
    },
}
