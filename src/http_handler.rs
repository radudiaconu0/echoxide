use crate::log::Log;
use crate::ws_handler::WSHandler;
use axum::extract::{ConnectInfo, Path};
use axum::response::IntoResponse;
use echoxide::WebSocketUpgrade;

use hyper::{HeaderMap, StatusCode};

use std::net::SocketAddr;

pub struct HttpHandler {}

impl HttpHandler {
    // pub fn new(server: &Rc<Server<T>>) -> Rc<Self> {
    //     let handler = Rc::new(Self {
    //         server: Rc::downgrade(server),
    //     });
    //     *server.http_handler.borrow_mut() = Some(handler.clone());
    //     handler
    // }
    pub async fn health_check() -> impl IntoResponse {
        "OK"
    }

    pub async fn channel(
        Path(app_id): Path<String>,
        Path(channel_name): Path<String>,
    ) -> impl IntoResponse {
        println!(
            "WebSocket connection for app {} and channel {}",
            app_id, channel_name
        );
    }

    pub async fn channels(Path(app_id): Path<u32>) -> impl IntoResponse {
        format!("Channels for app {}", app_id)
    }

    pub(crate) async fn ws_handler(
        Path(app_id): Path<String>,
        ws: WebSocketUpgrade,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ) -> impl IntoResponse {
        // finalize the upgrade process by returning upgrade callback.
        // we can customize the callback by sending additional info such as address.
        println!("WebSocket connection for app {}", app_id);
        ws.on_upgrade(move |socket| WSHandler::handle_socket(socket, addr))
    }

    pub async fn ready() -> impl IntoResponse {
        Log::info("Server is ready");
        "OK"
    }

    pub async fn events() -> impl IntoResponse {
        "Events"
    }

    pub fn send_json(&self, data: serde_json::Value, status: String) -> impl IntoResponse {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        (StatusCode::OK, headers, data.to_string())
    }
}
