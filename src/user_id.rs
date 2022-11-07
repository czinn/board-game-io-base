use serde::{Deserialize, Serialize};
use typescript_definitions::TypeScriptify;

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize, TypeScriptify)]
pub struct UserId(String);
