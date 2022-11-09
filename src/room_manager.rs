use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use serde_json::Value;
use tokio::sync::{mpsc, oneshot, watch};

use crate::error::Error;
use crate::game::Game;
use crate::ids::{ReconnectToken, UserId};
use crate::protocol::UserInfo;
use crate::result::Result;
use crate::room::{JoinInfo, Room};

type Responder<T> = oneshot::Sender<Result<T>>;

pub enum RoomUpdate {
    UserInfo { users: Vec<UserInfo> },
    RoomInfo { config: Value },
    GameInfo { view: Value },
}

#[allow(dead_code)]
pub struct Subscription {
    token: ReconnectToken,
    user_id: UserId,
    username: String,
    game_view: watch::Receiver<Option<Value>>,
}

enum RoomManagerMessage {
    JoinRoom {
        join_info: JoinInfo,
        resp: Responder<Subscription>,
    },
}

impl Debug for RoomManagerMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::JoinRoom { join_info, .. } => join_info.fmt(f),
        }
    }
}

#[derive(Clone)]
pub struct RoomManager<T: Game + Send + Sync> {
    tx: mpsc::Sender<RoomManagerMessage>,
    game_type: PhantomData<T>,
}

impl<T: Game + Send + Sync> RoomManager<T> {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::channel(32);
        tokio::spawn(async move {
            let mut room = Room::<T>::new();
            let mut view_watches: HashMap<
                UserId,
                (watch::Sender<Option<Value>>, watch::Receiver<Option<Value>>),
            > = HashMap::new();
            while let Some(message) = rx.recv().await {
                match message {
                    RoomManagerMessage::JoinRoom { join_info, resp } => {
                        let _ = match room.join_room(join_info) {
                            Err(err) => resp.send(Err(err)),
                            Ok(user_data) => {
                                let (_tx, rx) = view_watches
                                    .entry(user_data.id.clone())
                                    .or_insert_with(|| watch::channel(None));
                                resp.send(Ok(Subscription {
                                    token: user_data.token.clone(),
                                    user_id: user_data.id.clone(),
                                    username: user_data.username.clone(),
                                    game_view: rx.clone(),
                                }))
                            }
                        };
                    }
                }
            }
        });
        Self {
            tx,
            game_type: PhantomData,
        }
    }

    async fn join_room_aux(&self, join_info: JoinInfo) -> Result<Subscription> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(RoomManagerMessage::JoinRoom {
                join_info,
                resp: tx,
            })
            .await
            .unwrap();
        match rx.await {
            Ok(res) => res,
            Err(err) => Err(Error::TokioError(format!("{:?}", err))),
        }
    }

    pub async fn join_room(&self, username: String) -> Result<Subscription> {
        self.join_room_aux(JoinInfo::Username(username)).await
    }

    pub async fn rejoin_room(&self, token: ReconnectToken) -> Result<Subscription> {
        self.join_room_aux(JoinInfo::ReconnectToken(token)).await
    }
}
