use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::{error::Error, Result};

use crate::game::Game;
use crate::ids::*;
use crate::protocol::{ClientMessage, ServerMessage};
use crate::room_manager::{RoomManagerHandle, Subscription};

pub struct ClientHandler<S: AsyncRead + AsyncWrite + Unpin, T: Game> {
    ws: WebSocketStream<S>,
    room_id: RoomId,
    room_manager: RoomManagerHandle<T>,
    subscription: Subscription,
}

async fn send<S: AsyncRead + AsyncWrite + Unpin>(ws: &mut WebSocketStream<S>, server_message: &ServerMessage) -> Result<()> {
    ws.send(Message::text(serde_json::to_string(server_message).unwrap())).await
}

impl<S: AsyncRead + AsyncWrite + Unpin, T: Game> ClientHandler<S, T> {
    pub async fn new(rooms: Arc<Mutex<HashMap<RoomId, RoomManagerHandle<T>>>>, mut ws: WebSocketStream<S>) -> Result<Self> {
        while let Some(msg) = ws.next().await {
            let client_message: serde_json::Result<ClientMessage> = serde_json::from_str(msg?.to_text()?);
            if let Ok(client_message) = client_message {
                match client_message {
                    ClientMessage::JoinRoom { username, room } => {
                        let room_id = match room {
                            Some(room) => room,
                            None => RoomId::new(),
                        };
                        let room_manager = {
                            let mut rooms = rooms.lock().unwrap();
                            rooms.entry(room_id.clone()).or_insert_with(|| RoomManagerHandle::<T>::new()).clone()
                        };
                        match room_manager.join_room(username).await {
                            Ok(subscription) => {
                                return Ok(Self { ws, room_id, room_manager, subscription });
                            },
                            Err(err) => {
                                send(&mut ws, &ServerMessage::Error { message: err.to_string() }).await?
                            },
                        }
                    },
                    ClientMessage::RejoinRoom { token, room } => {
                        let room_manager = {
                            let rooms = rooms.lock().unwrap();
                            rooms.get(&room).cloned()
                        };
                        match room_manager {
                            Some(room_manager) => {
                                match room_manager.rejoin_room(token).await {
                                    Ok(subscription) => {
                                        return Ok(Self { ws, room_id: room, room_manager, subscription });
                                    },
                                    Err(err) => {
                                        send(&mut ws, &ServerMessage::Error { message: format!("{:?}", err) }).await?
                                    },
                                }
                            },
                            None => send(&mut ws, &ServerMessage::Error { message: "Room does not exist".to_string() }).await?,
                        }
                    },
                    _ => send(&mut ws, &ServerMessage::Error { message: "Must join room first".to_string() }).await?,
                }
            }
        }

        Err(Error::ConnectionClosed)
    }

    async fn handle_client_message(&mut self, client_message: ClientMessage) -> Result<()> {
        match client_message {
            ClientMessage::UpdateConfig { config } => {
                match self.room_manager.update_config(self.subscription.user_id.clone(), config).await {
                    Ok(()) => (),
                    Err(err) => send(&mut self.ws, &ServerMessage::Error { message: err.to_string() }).await?,
                }
            },
            ClientMessage::StartGame { player_mapping } => {
                match self.room_manager.start_game(self.subscription.user_id.clone(), player_mapping).await {
                    Ok(()) => (),
                    Err(err) => send(&mut self.ws, &ServerMessage::Error { message: err.to_string() }).await?,
                }
            },
            ClientMessage::DoAction { action } => {
                match self.room_manager.do_action(self.subscription.user_id.clone(), action).await {
                    Ok(()) => (),
                    Err(err) => send(&mut self.ws, &ServerMessage::InvalidAction { message: err.to_string() }).await?,
                }
            },
            _ => send(&mut self.ws, &ServerMessage::Error { message: "You're in a room".to_string() }).await?,
        }

        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        send(&mut self.ws, &ServerMessage::JoinResponse {
            room_id: self.room_id.clone(),
            token: self.subscription.token.clone(),
            user_id: self.subscription.user_id.clone(),
            username: self.subscription.username.clone(),
        }).await?;
        let mut room_watch = self.room_manager.watch_room();
        let mut users_watch = self.room_manager.watch_users();
        loop {
            tokio::select! {
                message = self.ws.next() => {
                    if let Some(msg) = message {
                        let client_message: serde_json::Result<ClientMessage> = serde_json::from_str(msg?.to_text()?);
                        if let Ok(client_message) = client_message {
                            self.handle_client_message(client_message).await?;
                        }
                    } else {
                        break;
                    }
                },
                view_updated = self.subscription.game_view.changed() => {
                    if let Ok(()) = view_updated {
                        let view = (*self.subscription.game_view.borrow()).clone();
                        match view {
                            Some(view) => send(&mut self.ws, &ServerMessage::GameInfo { view: view }).await?,
                            None => (),
                        }
                    }
                },
                room_updated = room_watch.changed() => {
                    if let Ok(()) = room_updated {
                        let config = (*room_watch.borrow()).clone();
                        match config {
                            Some(config) => send(&mut self.ws, &ServerMessage::RoomInfo { config }).await?,
                            None => (),
                        }
                    }
                },
                users_updated = users_watch.changed() => {
                    if let Ok(()) = users_updated {
                        let users = (*users_watch.borrow()).clone();
                        send(&mut self.ws, &ServerMessage::UserInfo { users }).await?;
                    }
                },
            };
        }

        Ok(())
    }
}