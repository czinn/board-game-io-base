use serde::{Deserialize, Serialize};

use board_game_io_base::error::Error;
use board_game_io_base::game::Game;
use board_game_io_base::ids::PlayerId;
use board_game_io_base::result::Result;
use board_game_io_base::server::Server;

#[derive(Serialize, Deserialize)]
pub enum Action {
    Incr,
    Decr,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MyGame {
    count: i64,
    players: Vec<PlayerId>,
}

impl Game for MyGame {
    type View = i64;
    type Action = Action;
    type Config = usize;

    fn new(config: Self::Config) -> Result<Self> {
        Ok(MyGame {
            count: 0,
            players: (0..config).map(|x| PlayerId(x.to_string())).collect(),
        })
    }

    fn players(&self) -> Vec<PlayerId> {
        self.players.clone()
    }

    fn view(&self, _player: Option<&PlayerId>) -> &Self::View {
        &self.count
    }

    fn do_action(&mut self, _player: &PlayerId, action: &Self::Action) -> Result<()> {
        let new_count = match *action {
            Self::Action::Incr => self.count + 1,
            Self::Action::Decr => self.count - 1,
        };
        if new_count.abs() > 10 {
            Err(Error::InvalidAction("count too high or low".to_string()))
        } else {
            self.count = new_count;
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    Server::<MyGame>::run("127.0.0.1:9002".to_string()).await;
}
