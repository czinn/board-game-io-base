use std::collections::HashMap;

use crate::game::Game;
use crate::user_id::UserId;

pub enum GameRoom<T: Game> {
    Lobby {
        // First user is the lobby leader
        users: Vec<UserId>,
        config: T::Config,
    },
    Game {
        game_state: T::State,
        player_mapping: HashMap<UserId, T::Player>,
    },
}

impl<T: Game> GameRoom<T> {
    pub fn new(user: UserId) -> Self {
        Self::Lobby {
            users: vec![user],
            config: T::Config::default(),
        }
    }

    pub fn update_config(&mut self, new_config: T::Config) {
        if let Self::Lobby { ref mut config, .. } = self {
            *config = new_config;
        }
    }

    pub fn start_game(&mut self) {
        if let Self::Lobby { config, .. } = self {
            *self = Self::Game {
                game_state: T::create(config).unwrap(),
                player_mapping: HashMap::new(),
            };
        }
    }
}
