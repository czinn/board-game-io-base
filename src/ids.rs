use rand::{
    distributions::{Alphanumeric, Uniform},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use ts_rs::TS;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PlayerId(pub u32);

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ReconnectToken(String);

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RoomId(String);

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UserId(pub u32);

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UserId({})", self.0)
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
