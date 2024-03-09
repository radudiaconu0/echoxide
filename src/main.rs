mod channels;
mod http_handler;
mod log;
mod message;
// mod namespace;
// mod options;
mod handle_client;

// mod adapters;
mod adapters;
mod app;
mod metrics;
// mod namespace;
mod options;
mod server;
mod token;
mod utils;
mod ws_handler;

use crate::server::Server;
use fastwebsockets::{FragmentCollector, WebSocket};
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let incoming = ws.read_frame().await?;
    // Always returns full messages
    assert!(incoming.fin);
    let server = Server::new().await;
    server.start().await;
}
