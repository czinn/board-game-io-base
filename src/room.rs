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
    next_user_id: UserId,
}

impl<T: Game> Room<T> {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            user_data: HashMap::new(),
            state: RoomState::Lobby {
                config: T::Config::default(),
            },
            next_user_id: UserId(0),
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
                    let user_id = self.next_user_id;
                    self.next_user_id = UserId(self.next_user_id.0 + 1);
                    if self.user_data.contains_key(&user_id) {
                        continue;
                    }
                    self.user_data.insert(
                        user_id,
                        UserData {
                            id: user_id,
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

    pub fn kick_user(&mut self, user: &UserId, target: &UserId) -> Result<()> {
        self.ensure_leader(user)?;
        match &self.state {
            RoomState::Lobby { .. } => (),
            RoomState::Game { player_mapping, .. } => {
                if player_mapping.contains_key(target) {
                    return Err(Error::UserIsPlayer);
                }
            }
        }
        let _ = self.user_data.remove(target).ok_or(Error::UserNotFound)?;
        self.users.retain(|u| *u != *target);
        Ok(())
    }

    pub fn reassign_player(
        &mut self,
        user: &UserId,
        from_user: &UserId,
        to_user: &UserId,
    ) -> Result<()> {
        self.ensure_leader(user)?;
        match &mut self.state {
            RoomState::Lobby { .. } => Err(Error::GameNotStarted),
            RoomState::Game { player_mapping, .. } => {
                if player_mapping.contains_key(to_user) {
                    return Err(Error::UserIsAlreadyPlayer(*to_user));
                }
                let player_id = player_mapping.remove(from_user);
                match player_id {
                    Some(player_id) => {
                        player_mapping.insert(*to_user, player_id);
                        Ok(())
                    }
                    None => return Err(Error::UserIsNotPlayer(*from_user)),
                }
            }
        }
    }

    pub fn start_game(&mut self, user: &UserId) -> Result<()> {
        self.ensure_leader(user)?;
        if let RoomState::Lobby { config, .. } = &self.state {
            let game_state = T::new(config.clone(), self.users.len() as u32)?;
            let players = HashSet::<PlayerId>::from_iter(T::players(&game_state).into_iter());
            if players.len() != self.users.len() {
                return Err(Error::WrongPlayerCount);
            }
            let player_mapping =
                HashMap::from_iter(self.users.clone().into_iter().zip(players.into_iter()));
            self.state = RoomState::Game {
                game_state,
                player_mapping,
            };
            Ok(())
        } else {
            Err(Error::GameAlreadyStarted)
        }
    }

    pub fn reset_to_lobby(&mut self, user: &UserId) -> Result<()> {
        self.ensure_leader(user)?;
        self.state = RoomState::Lobby { config: T::Config::default() };
        Ok(())
    }

    pub fn user_view<'a>(&'a self, user: &UserId) -> Result<T::View<'a>> {
        if let RoomState::Game {
            game_state,
            player_mapping,
        } = &self.state
        {
            Ok(T::view(&game_state, player_mapping.get(user).copied()))
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
                Some(player) => T::do_action(game_state, *player, action),
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
            next_user_id: _,
        } = &self;
        let player_mapping = match state {
            RoomState::Lobby { .. } => None,
            RoomState::Game { player_mapping, .. } => Some(player_mapping),
        };
        users
            .iter()
            .map(|id| {
                let user_data = user_data.get(&id).unwrap();
                UserInfo::new(
                    *id,
                    user_data.username.clone(),
                    *id == users[0],
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
