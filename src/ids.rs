use rand::{
    distributions::{Alphanumeric, Uniform},
    Rng,
};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerId(pub String);

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct ReconnectToken(String);

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct RoomId(String);

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    pub fn new() -> Self {
        Self(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(4)
                .map(char::from)
                .collect(),
        )
    }
}

impl ReconnectToken {
    pub fn new() -> Self {
        Self(
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect(),
        )
    }
}

impl RoomId {
    pub fn new() -> Self {
        Self(
            rand::thread_rng()
                .sample_iter(&Uniform::new_inclusive('A', 'Z'))
                .take(4)
                .map(char::from)
                .collect(),
        )
    }
}
