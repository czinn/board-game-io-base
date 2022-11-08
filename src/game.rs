use serde::{de::DeserializeOwned, Serialize};

use crate::result::Result;
use crate::ids::PlayerId;

pub trait Game {
    type State: Serialize + DeserializeOwned;
    type View: Serialize + DeserializeOwned;
    type Action: Serialize + DeserializeOwned;
    type Config: Default;

    fn create(_: &Self::Config) -> Result<Self::State>;
    fn players(_: &Self::State) -> Vec<PlayerId>;
    fn view(_: &Self::State, _: Option<&PlayerId>) -> Self::View;
    fn do_action(_: &mut Self::State, _: &PlayerId, _: &Self::Action) -> Result<()>;
}
