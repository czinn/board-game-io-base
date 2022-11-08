use std::marker::{PhantomData, Send, Sync};
use std::net::SocketAddr;

use tokio::runtime::Handle;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tokio_tungstenite::tungstenite::Result;
use futures_util::{SinkExt, StreamExt};

use crate::game::Game;

pub struct Server<T: Game + Send + Sync> {
    game_type: PhantomData<T>,
}

impl<T: Game + Send + Sync + 'static> Server<T> {
    async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
        if let Err(e) = Self::handle_connection(peer, stream).await {
            match e {
                Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                err => eprintln!("Error processing connection: {}", err),
            }
        }
    }

    async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
        let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

        println!("New WebSocket connection: {}", peer);

        while let Some(msg) = ws_stream.next().await {
            let msg = msg?;
            if msg.is_text() || msg.is_binary() {
                ws_stream.send(msg).await?;
            }
        }

        Ok(())
    }

    pub async fn run(addr: String) {
        let handle = Handle::current();
        let listener = TcpListener::bind(addr).await.expect("Can't listen");

        while let Ok((stream, _)) = listener.accept().await {
            let peer = stream.peer_addr().expect("connected streams should have a peer address");
            println!("Peer address: {}", peer);

            handle.spawn(Self::accept_connection(peer, stream));
        }
    }
}
