use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct RoomId(String);
