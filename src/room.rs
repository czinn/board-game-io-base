use std::collections::HashMap;
use std::collections::HashSet;

use serde_json::Value;

use crate::error::Error;
use crate::game::Game;
use crate::ids::{PlayerId, ReconnectToken, UserId};
use crate::protocol::UserInfo;
use crate::result::Result;

#[derive(Debug)]
pub enum JoinInfo {
    Username(String),
    ReconnectToken(ReconnectToken),
}

pub enum RoomState<T: Game> {
    Lobby {
        config: T::Config,
    },
    Game {
        game_state: T,
        player_mapping: HashMap<UserId, PlayerId>,
    },
}

pub struct UserData {
    pub id: UserId,
    pub username: String,
    pub token: ReconnectToken,
}

pub struct Room<T: Game> {
    // First user is the lobby leader
    users: Vec<UserId>,
    // Map may contain users that are not currently connected, but might reconnect later
    user_data: HashMap<UserId, UserData>,
    state: RoomState<T>,
}

impl<T: Game> Room<T> {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            user_data: HashMap::new(),
            state: RoomState::Lobby {
                config: T::Config::default(),
            },
        }
    }

    pub fn join_room(&mut self, join_info: JoinInfo) -> Result<&UserData> {
        match join_info {
            JoinInfo::Username(username) => {
                // Ensure no existing user has that name
                if self
                    .user_data
                    .values()
                    .any(|data| data.username == username)
                {
                    return Err(Error::UsernameInUse);
                }
                loop {
                    let user_id = UserId::new();
                    if self.user_data.contains_key(&user_id) {
                        continue;
                    }
                    self.user_data.insert(
                        user_id.clone(),
                        UserData {
                            id: user_id.clone(),
                            username,
                            token: ReconnectToken::new(),
                        },
                    );
                    let result = self.user_data.get(&user_id).unwrap();
                    self.users.push(user_id);
                    break Ok(result);
                }
            }
            JoinInfo::ReconnectToken(token) => {
                match self.user_data.values().find(|data| data.token == token) {
                    Some(data) => {
                        if !self.users.contains(&data.id) {
                            self.users.push(data.id.clone());
                        }
                        Ok(data)
                    }
                    None => Err(Error::InvalidReconnectToken),
                }
            }
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
        // TODO: consider getting extra users from [user_data] instead
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

    pub fn start_game(
        &mut self,
        user: &UserId,
        player_mapping: Option<HashMap<UserId, PlayerId>>,
    ) -> Result<()> {
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
            let game_state = T::new(config.clone())?;
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
                Some(player) => T::do_action(game_state, player, action),
                None => Err(Error::UserNotInGame),
            }
        } else {
            Err(Error::GameNotStarted)
        }
    }

    pub fn user_info(&self) -> Vec<UserInfo> {
        let Self {
            users,
            user_data,
            state,
        } = &self;
        let player_mapping = match state {
            RoomState::Lobby { .. } => None,
            RoomState::Game { player_mapping, .. } => Some(player_mapping),
        };
        user_data
            .iter()
            .map(|(id, user_data)| {
                UserInfo::new(
                    id.clone(),
                    user_data.username.clone(),
                    *id == users[0],
                    users.contains(&id),
                    match player_mapping {
                        Some(player_mapping) => player_mapping.get(&id).cloned(),
                        None => None,
                    },
                )
            })
            .collect()
    }

    pub fn lobby_info(&self) -> Option<Value> {
        match &self.state {
            RoomState::Lobby { config } => Some(serde_json::to_value(config).unwrap()),
            RoomState::Game { .. } => None,
        }
    }
}
