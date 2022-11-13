use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

use serde_json::Value;
use tokio::sync::{mpsc, oneshot, watch};

use crate::error::Error;
use crate::game::Game;
use crate::ids::*;
use crate::protocol::UserInfo;
use crate::result::Result;
use crate::room::{JoinInfo, Room};

type Responder<T> = oneshot::Sender<Result<T>>;

#[derive(Debug)]
pub struct Subscription {
    pub token: ReconnectToken,
    pub user_id: UserId,
    pub username: String,
    pub game_view: watch::Receiver<Option<Value>>,
}

#[derive(Debug)]
pub enum RoomManagerMessage {
    JoinRoom {
        join_info: JoinInfo,
        resp: Responder<Subscription>,
    },
    UpdateConfig {
        user_id: UserId,
        config: Value,
        resp: Responder<()>,
    },
    StartGame {
        user_id: UserId,
        player_mapping: Option<HashMap<UserId, PlayerId>>,
        resp: Responder<()>,
    },
    DoAction {
        user_id: UserId,
        action: Value,
        resp: Responder<()>,
    },
}

pub struct RoomManager<T: Game + Send + Sync + 'static> {
    room: Room<T>,
    message_rx: mpsc::Receiver<RoomManagerMessage>,
    room_tx: watch::Sender<Option<Value>>,
    users_tx: watch::Sender<Vec<UserInfo>>,
    view_watches: HashMap<UserId, (watch::Sender<Option<Value>>, watch::Receiver<Option<Value>>)>,
}

impl<T: Game + Send + Sync + 'static> RoomManager<T> {
    pub fn new(
        message_rx: mpsc::Receiver<RoomManagerMessage>,
        room_tx: watch::Sender<Option<Value>>,
        users_tx: watch::Sender<Vec<UserInfo>>,
    ) -> Self {
        let s = Self {
            room: Room::<T>::new(),
            message_rx,
            room_tx,
            users_tx,
            view_watches: HashMap::new(),
        };
        s.update_room();
        s
    }

    fn update_users(&self) {
        // TODO: error handling
        self.users_tx.send(self.room.user_info()).unwrap()
    }

    fn update_room(&self) {
        // TODO: error handling
        self.room_tx.send(self.room.lobby_info()).unwrap()
    }

    fn update_game(&self) {
        let Self {
            room, view_watches, ..
        } = &self;
        for (user_id, (tx, _rx)) in view_watches.iter() {
            let new_view = match room.user_view(user_id) {
                Ok(view) => Some(serde_json::to_value(view).unwrap()),
                Err(_) => None,
            };
            // TODO: error handling
            tx.send(new_view).unwrap()
        }
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.message_rx.recv().await {
            let mut users_dirty = false;
            let mut room_dirty = false;
            let mut game_dirty = false;
            match message {
                RoomManagerMessage::JoinRoom { join_info, resp } => {
                    let _ = match self.room.join_room(join_info) {
                        Err(err) => resp.send(Err(err)),
                        Ok(user_data) => {
                            let (_tx, rx) = self
                                .view_watches
                                .entry(user_data.id.clone())
                                .or_insert_with(|| watch::channel(None));
                            users_dirty = true;
                            resp.send(Ok(Subscription {
                                token: user_data.token.clone(),
                                user_id: user_data.id.clone(),
                                username: user_data.username.clone(),
                                game_view: rx.clone(),
                            }))
                        }
                    };
                }
                RoomManagerMessage::UpdateConfig {
                    user_id,
                    config,
                    resp,
                } => {
                    let result = match serde_json::from_value(config) {
                        Ok(config) => {
                            let result = self.room.update_config(&user_id, config);
                            if result.is_ok() {
                                room_dirty = true;
                            }
                            result
                        }
                        Err(_) => Err(Error::ParseFailure),
                    };
                    let _ = resp.send(result);
                }
                RoomManagerMessage::StartGame {
                    user_id,
                    player_mapping,
                    resp,
                } => {
                    let result = self.room.start_game(&user_id, player_mapping);
                    if result.is_ok() {
                        users_dirty = true;
                        room_dirty = true;
                        game_dirty = true;
                    }
                    let _ = resp.send(result);
                }
                RoomManagerMessage::DoAction {
                    user_id,
                    action,
                    resp,
                } => {
                    let result = match serde_json::from_value(action) {
                        Ok(action) => {
                            let result = self.room.user_action(&user_id, &action);
                            if result.is_ok() {
                                game_dirty = true;
                            }
                            result
                        }
                        Err(_) => Err(Error::ParseFailure),
                    };
                    let _ = resp.send(result);
                }
            }

            if users_dirty {
                self.update_users()
            }
            if room_dirty {
                self.update_room()
            }
            if game_dirty {
                self.update_game()
            }
        }
    }
}

#[derive(Clone)]
pub struct RoomManagerHandle<T: Game> {
    tx: mpsc::Sender<RoomManagerMessage>,
    room_watch: watch::Receiver<Option<Value>>,
    users_watch: watch::Receiver<Vec<UserInfo>>,
    game_type: PhantomData<T>,
}

impl<T: Game> RoomManagerHandle<T> {
    pub fn new() -> Self {
        let (tx, message_rx) = mpsc::channel(32);
        let (room_tx, room_watch) = watch::channel(None);
        let (users_tx, users_watch) = watch::channel(Vec::new());
        tokio::spawn(async move {
            let mut room_manager = RoomManager::<T>::new(message_rx, room_tx, users_tx);
            room_manager.run().await
        });
        Self {
            tx,
            room_watch,
            users_watch,
            game_type: PhantomData,
        }
    }

    async fn send_message<R, F>(&self, message: F) -> Result<R>
    where
        F: FnOnce(Responder<R>) -> RoomManagerMessage,
    {
        let (tx, rx) = oneshot::channel();
        let result = self.tx.send(message(tx)).await;
        match result {
            Ok(()) => (),
            Err(err) => return Err(Error::TokioError(err.to_string())),
        };
        match rx.await {
            Ok(res) => res,
            Err(err) => Err(Error::TokioError(err.to_string())),
        }
    }

    pub async fn join_room(&self, username: String) -> Result<Subscription> {
        self.send_message(|resp| RoomManagerMessage::JoinRoom {
            join_info: JoinInfo::Username(username),
            resp,
        })
        .await
    }

    pub async fn rejoin_room(&self, token: ReconnectToken) -> Result<Subscription> {
        self.send_message(|resp| RoomManagerMessage::JoinRoom {
            join_info: JoinInfo::ReconnectToken(token),
            resp,
        })
        .await
    }

    pub async fn update_config(&self, user_id: UserId, config: Value) -> Result<()> {
        self.send_message(|resp| RoomManagerMessage::UpdateConfig {
            user_id,
            config,
            resp,
        })
        .await
    }

    pub async fn start_game(
        &self,
        user_id: UserId,
        player_mapping: Option<HashMap<UserId, PlayerId>>,
    ) -> Result<()> {
        self.send_message(|resp| RoomManagerMessage::StartGame {
            user_id,
            player_mapping,
            resp,
        })
        .await
    }

    pub async fn do_action(&self, user_id: UserId, action: Value) -> Result<()> {
        self.send_message(|resp| RoomManagerMessage::DoAction {
            user_id,
            action,
            resp,
        })
        .await
    }

    pub fn watch_room(&self) -> watch::Receiver<Option<Value>> {
        self.room_watch.clone()
    }

    pub fn watch_users(&self) -> watch::Receiver<Vec<UserInfo>> {
        self.users_watch.clone()
    }
}
