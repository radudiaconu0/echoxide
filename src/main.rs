mod channels;
mod http_handler;
mod log;
mod message;
// mod namespace;
// mod options;
mod handle_client;

// mod adapters;
mod app;
// mod metrics;
mod namespace;
mod options;
mod server;
mod utils;
mod ws_handler;

use crate::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new().await;
    server.start().await;
}
