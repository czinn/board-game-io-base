use std::collections::HashMap;
use std::collections::HashSet;

use crate::error::Error;
use crate::game::Game;
use crate::result::Result;
use crate::ids::{PlayerId, RoomId, UserId};

pub enum RoomState<T: Game> {
    Lobby {
        config: T::Config,
    },
    Game {
        game_state: T,
        player_mapping: HashMap<UserId, PlayerId>,
    },
}

pub struct Room<T: Game> {
    #[allow(dead_code)]
    id: RoomId,
    // First user is the lobby leader
    users: Vec<UserId>,
    state: RoomState<T>,
}

impl<T: Game> Room<T> {
    pub fn new(room_id: &RoomId, user: &UserId) -> Self {
        Self {
            id: room_id.clone(),
            users: vec![user.clone()],
            state: RoomState::Lobby {
                config: T::Config::default(),
            },
        }
    }

    pub fn user_leader(&self) -> Result<&UserId> {
        if self.users.len() > 0 {
            Ok(&self.users[0])
        } else {
            Err(Error::EmptyLobby)
        }
    }

    pub fn active_users(&self) -> &Vec<UserId> {
        &self.users
    }

    pub fn all_users(&self) -> Vec<&UserId> {
        let mut users = HashSet::<&UserId>::from_iter(self.users.iter());
        match &self.state {
            RoomState::Lobby { .. } => (),
            RoomState::Game { player_mapping, .. } => users.extend(player_mapping.keys()),
        }
        users.into_iter().collect()
    }

    fn ensure_leader(&self, user: &UserId) -> Result<()> {
        if self.users.len() > 0 && self.users[0] == *user {
            Ok(())
        } else {
            Err(Error::UserNotLeader)
        }
    }

    pub fn update_config(&mut self, user: &UserId, new_config: T::Config) -> Result<()> {
        self.ensure_leader(user)?;
        if let RoomState::Lobby { ref mut config, .. } = self.state {
            *config = new_config;
            Ok(())
        } else {
            Err(Error::GameAlreadyStarted)
        }
    }

    pub fn start_game(&mut self, user: &UserId, player_mapping: Option<HashMap<UserId, PlayerId>>) -> Result<()> {
        self.ensure_leader(user)?;
        if let RoomState::Lobby { config, .. } = &self.state {
            match player_mapping {
                Some(ref player_mapping) => {
                    // Verify that all users in the mapping are in the lobby.
                    let users = HashSet::<UserId>::from_iter(self.users.clone().into_iter());
                    if !player_mapping.keys().all(|user_id| users.contains(user_id)) {
                        return Err(Error::InvalidPlayerMapping);
                    }
                }
                None => (),
            }
            let game_state = T::new(&config)?;
            let players = HashSet::<PlayerId>::from_iter(T::players(&game_state).into_iter());
            let player_mapping = match player_mapping {
                Some(player_mapping) => {
                    if !player_mapping
                        .values()
                        .all(|player| players.contains(player))
                    {
                        return Err(Error::InvalidPlayerMapping);
                    }
                    player_mapping
                }
                None => {
                    if players.len() != self.users.len() {
                        return Err(Error::WrongPlayerCount);
                    }
                    // TODO: Shuffle one of the lists
                    HashMap::from_iter(self.users.clone().into_iter().zip(players.into_iter()))
                }
            };
            self.state = RoomState::Game {
                game_state,
                player_mapping,
            };
            Ok(())
        } else {
            Err(Error::GameAlreadyStarted)
        }
    }

    pub fn user_view(&self, user: &UserId) -> Result<T::View> {
        if let RoomState::Game {
            game_state,
            player_mapping,
        } = &self.state
        {
            Ok(T::view(&game_state, player_mapping.get(user)))
        } else {
            Err(Error::GameNotStarted)
        }
    }

    pub fn user_action(&mut self, user: &UserId, action: &T::Action) -> Result<()> {
        if let RoomState::Game {
            ref mut game_state,
            player_mapping,
        } = &mut self.state
        {
            match player_mapping.get(user) {
                Some(player) => {
                    T::do_action(game_state, player, action)
                },
                None => Err(Error::UserNotInGame),
            }
        } else {
            Err(Error::GameNotStarted)
        }
    }
}
