use serde::{de::DeserializeOwned, Serialize};

use crate::result::Result;
use crate::ids::PlayerId;

pub trait Game: Serialize + DeserializeOwned {
    type View: Serialize + DeserializeOwned;
    type Action: Serialize + DeserializeOwned;
    type Config: Default;

    fn new(_: &Self::Config) -> Result<Self>;
    fn players(&self) -> Vec<PlayerId>;
    fn view(&self, _: Option<&PlayerId>) -> Self::View;
    fn do_action(&mut self, _: &PlayerId, _: &Self::Action) -> Result<()>;
}
