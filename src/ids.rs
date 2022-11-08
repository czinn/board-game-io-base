use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct PlayerId(String);

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct ReconnectToken(String);

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct RoomId(String);

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct UserId(String);
