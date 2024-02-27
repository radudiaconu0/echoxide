use crate::http_handler::HttpHandler;
use crate::log::Log;
use crate::ws_handler::WSHandler;
use axum::routing::get;
use axum::Router;
use fred::types::Options;
use std::net::SocketAddr;
use std::sync::{Arc, Weak};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::Mutex;

pub struct Server {
    addr: String,
    router: Router,
    pub closing: bool,
    options: Option<Options>,
    ws_handler: Mutex<Option<Arc<WSHandler>>>,
}

impl Server {
    pub(crate) async fn new(addr: String) -> Arc<Self> {
        let router = Router::new()
            .route("/health", get(HttpHandler::health_check))
            .route("/app/:appId", get(HttpHandler::ws_handler))
            .route(
                "/apps/:appId/channels/:channelName",
                get(HttpHandler::channel),
            )
            .route("/apps/:appId/channels", get(HttpHandler::channels));
        let server = Arc::new(Server {
            addr,
            router,
            closing: false,
            options: None,
            ws_handler: Mutex::new(None),
        });
        let ws_handler = Arc::new(WSHandler {
            server: Arc::downgrade(&server), // Create a Weak reference from the server
        });
        server.ws_handler.lock().await.replace(ws_handler);
        server
    }
    pub(crate) async fn start(&self) {
        let server = TcpListener::bind(&self.addr).await.unwrap();
        Log::info(&format!("Listening on {}", self.addr));
        axum::serve(
            server,
            self.router
                .clone()
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    }

    async fn stop(mut self) {
        self.closing = true;
        Log::info("Shutting down server");
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
    }
    pub fn get_instance(self) -> Self {
        self
    }
}
