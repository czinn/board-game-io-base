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

#[derive(Serialize, Deserialize)]
pub struct MyGame(i64);

impl Game for MyGame {
    type View = i64;
    type Action = Action;
    type Config = ();

    fn new(() : &()) -> Result<Self> {
        Ok(MyGame(0))
    }

    fn players(&self) -> Vec<PlayerId> {
        vec![PlayerId("0".to_string())]
    }

    fn view(&self, _player: Option<&PlayerId>) -> Self::View {
        self.0
    }

    fn do_action(&mut self, _player: &PlayerId, action: &Self::Action) -> Result<()> {
        match *action {
            Self::Action::Incr => self.0 += 1,
            Self::Action::Decr => self.0 -= 1,
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    Server::<MyGame>::run("127.0.0.1:9002".to_string()).await;
}
