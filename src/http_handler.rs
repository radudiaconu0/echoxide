use crate::log::Log;
use crate::ws_handler::WSHandler;
use axum::extract::{ConnectInfo, Path, Query};
use axum::response::{IntoResponse, Response};
use echoxide::WebSocketUpgrade;

use hyper::{HeaderMap, StatusCode};

// use crate::metrics::metrics_trait::MetricsTrait;
use crate::message::PusherApiMessage;
use crate::metrics::metrics_trait::MetricsTrait;
use crate::server::Server;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};
use std::net::SocketAddr;
use std::sync::{Arc, Weak};
use sysinfo::System;

pub struct HttpHandler {
    pub(crate) server: Weak<Server>,
}

impl HttpHandler {
    pub fn new(server: Server) -> Self {
        Self {
            server: Weak::new(),
        }
    }
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

    pub async fn ready() -> impl IntoResponse {
        Log::info("Server is ready".to_string());
        "OK"
    }

    pub async fn events(&self, Json(payload): Json<PusherApiMessage>) -> impl IntoResponse {
        Log::info(to_string_pretty(&payload).unwrap());
    }

    pub fn send_json(
        data: serde_json::Value,
        status: StatusCode,
    ) -> (StatusCode, HeaderMap, String) {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        (status, headers, data.to_string())
    }

    pub async fn usage() -> impl IntoResponse {
        let mut sys = System::new_all();
        sys.refresh_memory();
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let free_memory = total_memory - used_memory;
        let percent_usage = (used_memory as f64 / total_memory as f64) * 100.0;
        HttpHandler::send_json(
            json!({
                "total_memory": total_memory,
                "used_memory": used_memory,
                "free_memory": free_memory,
                "percent_usage": percent_usage,
            }),
            StatusCode::OK,
        )
    }

    pub async fn metrics(&self, query: Query<PrometheusQuery>) -> impl IntoResponse {
        let server = self.server.upgrade();
        if let Some(server) = server {
            let metrics = server.metrics.lock().await;
            if let Some(metrics) = metrics.as_ref() {
                if let Some(json) = query.json {
                    if json {
                        return match metrics.get_metrics_as_json().await {
                            Ok(metrics) => (StatusCode::OK, HeaderMap::new(), metrics.to_string())
                                .into_response(),
                            Err(e) => (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                HeaderMap::new(),
                                format!("Error: {}", e),
                            )
                                .into_response(),
                        };
                    }
                }
                return match metrics.get_metrics_as_plaintext().await {
                    Ok(metrics) => (StatusCode::OK, HeaderMap::new(), metrics).into_response(),
                    Err(e) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        HeaderMap::new(),
                        "Error: No metrics".to_string(),
                    )
                        .into_response(),
                };
            }
        }
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            HeaderMap::new(),
            "No metrics".to_string(),
        )
            .into_response()
    }
    pub async fn error() -> impl IntoResponse {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            HeaderMap::new(),
            "Internal server error".to_string(),
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrometheusQuery {
    json: Option<bool>,
}
