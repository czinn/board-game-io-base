use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

use crate::reconnect_token::ReconnectToken;
use crate::room_id::RoomId;
use crate::user_id::UserId;

// Message from the server to the client.
#[derive(Serialize, Deserialize, TypeScriptify)]
#[serde(tag = "type")]
pub enum ServerMessage {
    RegisterResponse {
        token: ReconnectToken,
        username: String,
        user_id: UserId,
    },
}

// Message from the client to the server.
#[derive(Serialize, Deserialize, TypeScriptify)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Register {
        name: String,
    },
    Reconnect {
        token: ReconnectToken,
    },
}
