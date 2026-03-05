use futures_util::{FutureExt, SinkExt, StreamExt};
use shared::protocol::{ClientMessage, ServerMessage};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub struct ClientConnection {
    ws: WebSocketStream<TcpStream>,
}

impl ClientConnection {
    pub fn new(ws: WebSocketStream<TcpStream>) -> Self {
        ClientConnection { ws }
    }

    pub async fn send(&mut self, msg: &ServerMessage) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(msg)?;
        self.ws.send(Message::Text(json.into())).await?;
        Ok(())
    }

    pub async fn try_recv(&mut self) -> Option<ClientMessage> {
        match self.ws.next().now_or_never() {
            Some(Some(Ok(Message::Text(text)))) => {
                serde_json::from_str(&text).ok()
            }
            _ => None,
        }
    }

    pub async fn recv(&mut self) -> Option<ClientMessage> {
        loop {
            match self.ws.next().await {
                Some(Ok(Message::Text(text))) => {
                    if let Ok(msg) = serde_json::from_str::<ClientMessage>(&text) {
                        return Some(msg);
                    }
                }
                Some(Ok(Message::Close(_))) | None => return None,
                Some(Err(_)) => return None,
                _ => continue,
            }
        }
    }
}
