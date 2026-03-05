mod game;
mod map;
mod network;
mod recording;
mod vehicle;

use shared::protocol::{ClientMessage, ServerMessage};
use tokio::net::TcpListener;
use tokio::time::{interval, Duration};

use crate::game::Game;
use crate::network::ClientConnection;

const TICK_RATE: u64 = 60;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server listening on ws://0.0.0.0:8080");

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("New connection from {addr}");
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake failed: {e}");
            return;
        }
    };

    let mut client = ClientConnection::new(ws_stream);
    let mut game = Game::new();

    // Add player
    let player_id = game.add_player();
    let joined_msg = ServerMessage::Joined { id: player_id };
    if client.send(&joined_msg).await.is_err() {
        return;
    }

    // Game loop at fixed tick rate
    let mut tick_interval = interval(Duration::from_micros(1_000_000 / TICK_RATE));

    loop {
        tick_interval.tick().await;

        // Read all available input messages (non-blocking)
        loop {
            match client.try_recv().await {
                Some(ClientMessage::Input { left, right }) => {
                    game.set_player_input(left, right);
                }
                None => break,
            }
        }

        // Step physics
        game.step();

        // Send state
        let state_msg = game.get_state_message();
        if client.send(&state_msg).await.is_err() {
            println!("Client disconnected");
            break;
        }
    }

    game.remove_player();
}
