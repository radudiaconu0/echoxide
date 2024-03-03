use crate::http_handler::{HttpHandler, PrometheusQuery};
use crate::log::Log;
// use crate::metrics::prometheus_metrics_driver::PrometheusMetricsDriver;
use crate::options::{
    Adapter, AppManager, ArrayAppManager, CacheAppManager, ClusterAdapter, Metrics,
    MySQLAppManager, NatsAdapter, Options, Prometheus, RedisAdapter,
};
use crate::ws_handler::WSHandler;
use tracing_subscriber;

use axum::routing::{get, post, Route};
use axum::Router;
use axum::{
    http::{header::HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};

use std::fmt::format;
use std::net::SocketAddr;
use std::sync::{Arc, Weak};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::Mutex;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct Server {
    pub closing: bool,
    options: Option<Options>,
    ws_handler: Mutex<Option<Arc<WSHandler>>>,
    // pub(crate) metrics: Mutex<Option<Arc<PrometheusMetricsDriver>>>,
    // http_handler: Mutex<Option<Arc<HttpHandler>>>,
}

impl Server {
    pub(crate) async fn new() -> Arc<Self> {
        let options = Options {
            adapter: Adapter {
                driver: "".to_string(),
                redis: RedisAdapter {
                    request_timeout: 0,
                    prefix: "".to_string(),
                    redis_pub_options: Default::default(),
                    redis_sub_options: Default::default(),
                    cluster_mode: false,
                },
                cluster: ClusterAdapter { request_timeout: 0 },
                nats: NatsAdapter {
                    request_timeout: 0,
                    prefix: "".to_string(),
                    servers: vec![],
                    user: None,
                    password: None,
                    token: None,
                    timeout: 0,
                    nodes_number: None,
                },
            },
            app_manager: AppManager {
                driver: "".to_string(),
                array: ArrayAppManager { apps: vec![] },
                cache: CacheAppManager {
                    enabled: false,
                    ttl: 0,
                },
                mysql: MySQLAppManager {
                    table: "".to_string(),
                    version: "".to_string(),
                },
            },
            debug: true,
            port: 6001,
            metrics: Metrics {
                enabled: false,
                driver: String::from("prometheus"),
                host: String::from("0.0.0.0"),
                prometheus: Prometheus {
                    prefix: "echoxide_".to_string(),
                },
                port: 9601,
            },
        };
        let server = Arc::new(Server {
            closing: false,
            options: Some(options),
            ws_handler: Mutex::new(None),
            // metrics: Mutex::new(None),
            // http_handler: Mutex::new(None),
        });
        let ws_handler = Arc::new(WSHandler {
            server: Arc::downgrade(&server), // Create a Weak reference from the server
        });
        // let metrics = Arc::new(PrometheusMetricsDriver::new(Arc::downgrade(&server)));
        // let http_handler = Arc::new(HttpHandler {
        //     server: Arc::downgrade(&server),
        // });
        server.ws_handler.lock().await.replace(ws_handler);
        // server.metrics.lock().await.replace(metrics);
        // server.http_handler.lock().await.replace(http_handler);
        server
    }
    pub async fn start(&self) {
        Log::br();
        let options = self.options.as_ref().unwrap();
        if options.debug {
            Log::info("📡 echoxide initialization....".to_string());
            Log::info("⚡ Initializing the HTTP API & Websockets Server...".to_string());
        }
        self.start_main_server().await;
    }

    async fn stop(&mut self) {
        self.closing = true;
        Log::br();
        Log::warning(
            "🚫 New users cannot connect to this instance anymore. Preparing for signaling...",
        );
        Log::warning(
            "⚡ The server is closing and signaling the existing connections to terminate.",
        );
        Log::br();
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

    // pub async fn metrics_server(&self) -> Router {
    //     let http_handler = self.http_handler.lock().await.clone().unwrap(); // Clone the Arc
    //     Router::new()
    //         .route("/usage", get(HttpHandler::usage))
    //         .route(
    //             "/metrics",
    //             get(move |prometheus_query: Query<PrometheusQuery>| async move {
    //                 http_handler.clone().metrics(prometheus_query).await
    //             }),
    //         )
    // }
    // async fn start_metrics_server(&self) {
    //     let app = self.metrics_server().await;
    //     let options = self.options.as_ref().unwrap();
    //     let addr = format!("{}:{}", options.metrics.host, options.metrics.port);
    //     let listener = TcpListener::bind(addr).await.unwrap();
    //     Log::success_title(format!(
    //         "🌠 Prometheus /metrics endpoint is available on port {}",
    //         options.metrics.port
    //     ));
    //     tracing::debug!("listening on {}", listener.local_addr().unwrap());
    //     axum::serve(listener, app).await.unwrap();
    // }

    pub async fn start_main_server(&self) {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        let router = Router::new()
            .route("/ws", get(WSHandler::ws_handler))
            .route("/health", get(HttpHandler::health_check))
            .route(
                "/apps/:app_id/channels/:channel_name",
                get(HttpHandler::channel),
            )
            .route("/apps/:app_id/channels", get(HttpHandler::channels))
            .route("/ready", get(HttpHandler::ready))
            .route("/events", post(HttpHandler::events))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true)),
            );

        let server = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 6001)))
            .await
            .unwrap();
        tracing::debug!("listening on {}", server.local_addr().unwrap());
        Log::success_title("🎉 Server is up and running!".to_string());
        Log::success_title(format!(
            "📡 The Websockets server is available at 127.0.0.1:{}",
            self.options.as_ref().unwrap().port
        ));
        Log::success_title(format!(
            "🔗 The HTTP API server is available at http://127.0.0.1:{}",
            self.options.as_ref().unwrap().port
        ));
        Log::br();
        axum::serve(server, router).await.unwrap();
    }
}
