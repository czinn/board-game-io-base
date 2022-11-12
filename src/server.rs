use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Handle;
use tokio_tungstenite::tungstenite::Result;
use tokio_tungstenite::{accept_async, tungstenite::Error};

use crate::game::Game;
use crate::ids::*;
use crate::room_manager::RoomManagerHandle;
use crate::client_handler::ClientHandler;

type Rooms<T> = Arc<Mutex<HashMap<RoomId, RoomManagerHandle<T>>>>;

pub struct Server<T: Game> {
    game_type: PhantomData<T>
}

impl<T: Game> Server<T> {
    async fn accept_connection(peer: SocketAddr, stream: TcpStream, rooms: Rooms<T>) {
        if let Err(e) = Self::handle_connection(peer, stream, rooms).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => eprintln!("Error processing connection: {}", err),
            }
        }
    }

    async fn handle_connection(peer: SocketAddr, stream: TcpStream, rooms: Rooms<T>) -> Result<()> {
        let ws_stream = accept_async(stream).await.expect("Failed to accept");

        println!("New WebSocket connection: {}", peer);

        let mut client = ClientHandler::new(rooms, ws_stream).await?;
        client.run().await
    }

    pub async fn run(addr: String) {
        let handle = Handle::current();
        let listener = TcpListener::bind(addr).await.expect("Can't listen");
        let rooms: Arc<Mutex<HashMap<RoomId, RoomManagerHandle<T>>>> = Arc::new(Mutex::new(HashMap::new()));

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream
                .peer_addr()
                .expect("connected streams should have a peer address");
            println!("Peer address: {}", peer);

            handle.spawn(Self::accept_connection(peer, stream, rooms.clone()));
        }
    }
}
