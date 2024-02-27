mod channels;
mod http_handler;
mod log;
mod message;
// mod namespace;
// mod options;
mod handle_client;

mod adapters;
mod namespace;
mod server;
mod utils;
mod ws_handler;

use crate::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new("0.0.0.0:3000".to_string()).await;
    server.start().await;
}
