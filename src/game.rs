use crate::result::Result;

pub trait Game {
    type State;
    type View;
    type Action;
    type Config: Default;
    type Player;

    fn create(_: &Self::Config) -> Result<Self::State>;
    fn players(_: &Self::State) -> Vec<Self::Player>;
    fn view(_: &Self::State, _: &Self::Player) -> Self::View;
    fn do_action(_: &mut Self::State, _: &Self::Player, _: &Self::Action) -> Result<()>;
}
