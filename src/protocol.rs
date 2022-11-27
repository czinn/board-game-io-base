use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::ids::*;

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct UserInfo {
    pub id: UserId,
    pub username: String,
    pub leader: bool,
    pub player_id: Option<PlayerId>,
}

impl UserInfo {
    pub fn new(id: UserId, username: String, leader: bool, player_id: Option<PlayerId>) -> Self {
        Self {
            id,
            username,
            leader,
            player_id,
        }
    }
}

// Message from the server to the client.
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type", rename_all = "snake_case")]
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
    InvalidateToken {
        token: ReconnectToken,
    },
    UserInfo {
        users: Vec<UserInfo>,
    },
    RoomInfo {
        #[ts(type = "any")]
        config: Value,
    },
    GameInfo {
        #[ts(type = "any")]
        view: Value,
    },
    GameViewDiff {
        #[ts(type = "any")]
        diff: Value,
    },
    InvalidAction {
        message: String,
    },
}

// Message from the client to the server.
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type", rename_all = "snake_case")]
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
        #[ts(type = "any")]
        config: Value,
    },
    KickUser {
        user: UserId,
    },
    ReassignPlayer {
        from_user: UserId,
        to_user: UserId,
    },
    StartGame,
    DoAction {
        #[ts(type = "any")]
        action: Value,
    },
    GameViewRequest,
    ResetToLobby,
}
