use crate::channels::presence_channel_manager::PresenceMemberInfo;
use crate::log::Log;
use crate::message;
use crate::message::{PusherMessage, UWebSocketMessage};
use crate::server::Server;
use axum::extract::{ConnectInfo, Path, Query, State};
use axum::response::IntoResponse;
use echoxide::{WebSocketUpgrade, WS};
use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::net::SocketAddr;
use std::sync::{Arc, Weak};
use web_socket::Event;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
}

impl Eq for WebSocket {}

impl PartialEq for WebSocket {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct WebSocket {
    pub(crate) ws: WS,
    pub(crate) id: Option<String>,
    pub app_key: Option<String>,
    pub subscribed_channels: Option<Vec<String>>,
    pub presence_channels: Option<HashMap<String, PresenceMemberInfo>>,
    pub(crate) user: Option<User>,
}

impl Hash for WebSocket {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl WebSocket {
    pub fn new(ws: WS) -> Self {
        WebSocket {
            ws,
            id: None,
            app_key: None,
            subscribed_channels: None,
            presence_channels: None,
            user: None,
        }
    }

    pub async fn send_json(&mut self, data: serde_json::Value) {
        Log::websocket_title("Sending message to client");
        Log::websocket(serde_json::to_string_pretty(&data).unwrap().as_str());
        let message = match serde_json::to_string(&data) {
            Ok(message) => message,
            Err(e) => {
                Log::websocket(&format!("Error: {}", e));
                return;
            }
        };
        self.ws
            .send(message.as_str())
            .await
            .expect("TODO: panic message");
    }
    pub async fn stop(&mut self, socket: WS) {
        socket.close("Closed").await.expect("TODO: panic message");
    }
}

pub struct WSHandler {
    pub(crate) server: Weak<Server>,
}

impl WSHandler {
    pub fn new() -> Self {
        Self {
            server: Weak::new(),
        }
    }
    pub async fn on_open(&mut self, ws: &mut WebSocket) {
        Log::websocket_title("WebSocket connection opened");
        ws.id = Some(Self::generate_socket_id());
        ws.subscribed_channels = Some(Vec::new());
        ws.presence_channels = Some(HashMap::new());
        ws.id = Some(Self::generate_socket_id());
        if let Some(server) = self.server.upgrade() {
            if server.closing {
                ws.send_json(serde_json::json!({
                    "event": "pusher:error",
                    "data": serde_json::json!({
                        "code": 4200,
                        "message": "Server is closing. Please reconnect shortly.",
                    }),
                }))
                .await;
            }
        }
        let broadcast_message = serde_json::json!({
            "event": "pusher:connection_established",
            "data": serde_json::json!({
                "socket_id": ws.id.as_ref().unwrap(),
                "activity_timeout": 120,
            }),
        });
        ws.send_json(broadcast_message).await;
    }

    fn generate_socket_id() -> String {
        let mut rng = rand::thread_rng(); // Get a random number generator

        // Define min and max as u64, since Rust requires specifying the integer type
        let min: u64 = 0;
        let max: u64 = 10000000000;

        // Rust's rand crate handles generating a random number between min and max differently
        let mut random_number = |min: u64, max: u64| -> u64 { rng.gen_range(min..=max) };

        // Format the random numbers into a String with a dot separator
        format!("{}.{}", random_number(min, max), random_number(min, max))
    }

    pub(crate) fn on_close(&mut self, code: u16, message: String) {
        Log::websocket("❌ Connection closed:");
        Log::websocket(&format!("Code: {}", code));
        Log::websocket(&format!("Message: {}", message));
    }

    pub(crate) async fn on_message(&mut self, message: PusherMessage, ws: &mut WebSocket) {
        Log::websocket_title("Received message from client");
        match message.data {
            Some(data) => {
                Log::websocket(serde_json::to_string_pretty(&data).unwrap().as_str());
            }
            None => {
                Log::websocket("No data");
            }
        }
        match message.event {
            Some(event) => match event.as_str() {
                "pusher:subscribe" => {
                    Log::websocket("Subscribing to channel");
                    ws.send_json(serde_json::json!({
                        "event": "pusher_internal:subscription_succeeded",
                        "data": serde_json::json!({}),
                    }))
                    .await;
                }
                "pusher:unsubscribe" => {
                    ws.send_json(serde_json::json!({
                        "event": "pusher:unsubscribe",
                        "data": serde_json::json!({}),
                    }))
                    .await;
                    Log::websocket("Unsubscribing from channel");
                }
                "pusher:ping" => {
                    ws.send_json(serde_json::json!({
                        "event": "pusher:pong",
                        "data": serde_json::json!({}),
                    }))
                    .await;
                }
                "pusher:pong" => {
                    Log::websocket("Received pong");
                    ws.send_json(serde_json::json!({
                        "event": "pusher:ping",
                        "data": serde_json::json!({}),
                    }))
                    .await;
                }
                _ => {
                    Log::websocket("No event");
                }
            },
            None => {
                Log::websocket("No event");
            }
        }
    }

    pub async fn handle_pong(&mut self) {
        Log::websocket_title("Received pong");
    }

    pub async fn handle_ping(&mut self) {
        Log::websocket_title("Received ping");
    }

    pub async fn handle_socket(socket: WS, who: SocketAddr) {
        println!("New WebSocket connection: {}", who);
        let mut ws = WebSocket::new(socket);
        let mut ws_handler = WSHandler::new();
        ws_handler.on_open(&mut ws).await;
        while let Ok(ev) = ws.ws.recv().await {
            match ev {
                Event::Data { ty, data } => {
                    println!("Data: {:#?}", ty);
                    let data = String::from_utf8(data.to_vec()).unwrap();
                    let pusher_message: PusherMessage = serde_json::from_str(&data).unwrap();
                    ws_handler.on_message(pusher_message, &mut ws).await;
                }
                Event::Ping(_) => {
                    ws_handler.handle_ping().await;
                }
                Event::Pong(_) => {
                    ws_handler.handle_pong().await;
                }
                Event::Error(_) => {
                    Log::websocket("Error");
                }
                Event::Close { .. } => {
                    Log::websocket_title("❌ Connection closed:");
                }
            }
        }
    }

    pub async fn ws_handler(
        Path(app_id): Path<String>,
        query: Query<PusherWebsocketQuery>,
        ws: WebSocketUpgrade,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ) -> impl IntoResponse {
        Log::info(format!(
            "WebSocket connection for app {}. Protocol: {}, client: {}, version: {}, flash: {}",
            app_id,
            query.protocol.unwrap_or(0),
            query.client.as_deref().unwrap_or(""),
            query.version.as_deref().unwrap_or(""),
            query.flash.unwrap_or(false)
        ));
        ws.on_upgrade(move |socket| WSHandler::handle_socket(socket, addr))
    }
}

#[derive(Debug, serde::Deserialize, Serialize)]
pub struct PusherWebsocketQuery {
    protocol: Option<u8>,
    client: Option<String>,
    version: Option<String>,
    flash: Option<bool>,
}
