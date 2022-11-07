use std::hash::Hash;

use serde::{de::DeserializeOwned, Serialize};

use crate::result::Result;

pub trait Game {
    type State: Serialize + DeserializeOwned;
    type View: Serialize + DeserializeOwned;
    type Action: Serialize + DeserializeOwned;
    type Config: Default;
    type Player: Eq + PartialEq + Hash + Clone + Serialize + DeserializeOwned;

    fn create(_: &Self::Config) -> Result<Self::State>;
    fn players(_: &Self::State) -> Vec<Self::Player>;
    fn view(_: &Self::State, _: Option<&Self::Player>) -> Self::View;
    fn do_action(_: &mut Self::State, _: &Self::Player, _: &Self::Action) -> Result<()>;
}
