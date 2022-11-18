use serde::{de::DeserializeOwned, Serialize};

use crate::ids::PlayerId;
use crate::result::Result;

pub trait Game: Serialize + Send + Sync + Sized + Clone + 'static {
    type View<'a>: Serialize;
    type Action: Serialize + DeserializeOwned;
    type Config: Default + Clone + Send + Sync + Serialize + DeserializeOwned;

    fn new(_: Self::Config, players: u32) -> Result<Self>;
    fn players(&self) -> Vec<PlayerId>;
    fn view<'a>(&'a self, _: Option<&PlayerId>) -> Self::View<'a>;
    fn do_action(&mut self, _: &PlayerId, _: &Self::Action) -> Result<()>;
}
