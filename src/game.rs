use serde::{de::DeserializeOwned, Serialize};

use crate::ids::PlayerId;
use crate::result::Result;

pub trait Game: Serialize + Send + Sync + Sized + Clone + 'static {
    type View: Serialize + DeserializeOwned;
    type Action: Serialize + DeserializeOwned;
    type Config: Default + Clone + Send + Sync + Serialize + DeserializeOwned;

    fn new(_: Self::Config) -> Result<Self>;
    fn players(&self) -> Vec<PlayerId>;
    fn view(&self, _: Option<&PlayerId>) -> Self::View;
    fn do_action(&mut self, _: &PlayerId, _: &Self::Action) -> Result<()>;
}
