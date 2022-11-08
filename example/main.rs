use serde::{Deserialize, Serialize};

use board_game_io_base::game::Game;
use board_game_io_base::result::Result;
use board_game_io_base::ids::PlayerId;
use board_game_io_base::server::Server;

#[derive(Serialize, Deserialize)]
pub enum Action {
    Incr,
    Decr,
}

pub struct MyGame {}

impl Game for MyGame {
    type State = i64;
    type View = i64;
    type Action = Action;
    type Config = ();

    fn create(() : &()) -> Result<Self::State> {
        Ok(0)
    }

    fn players(state: &Self::State) -> Vec<PlayerId> {
        vec![PlayerId("0".to_string())]
    }

    fn view(state: &Self::State, player: Option<&PlayerId>) -> Self::View {
        *state
    }

    fn do_action(state: &mut Self::State, player: &PlayerId, action: &Self::Action) -> Result<()> {
        match *action {
            Self::Action::Incr => *state += 1,
            Self::Action::Decr => *state -= 1,
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    Server::<MyGame>::run("127.0.0.1:9002".to_string()).await;
}
