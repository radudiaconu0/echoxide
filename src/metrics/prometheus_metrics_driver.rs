use crate::metrics::metrics_trait::MetricsTrait;
use crate::server::Server;
use crate::utils::Utils;
use crate::ws_handler::WebSocket;
use metrics_exporter_prometheus;
use prometheus;
use prometheus::core::GenericGauge;
use prometheus::proto::MetricFamily;
use prometheus::{CounterVec, Encoder, Gauge, GaugeVec, HistogramVec, Registry, TextEncoder};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use tokio::runtime::Runtime;

struct PrometheusMetrics {
    registry: Registry,
    connected_sockets: GaugeVec,
    new_connections_total: CounterVec,
    new_disconnections_total: CounterVec,
    socket_bytes_received: CounterVec,
    socket_bytes_transmitted: CounterVec,
    ws_messages_received: CounterVec,
    ws_messages_sent: CounterVec,
    http_bytes_received: CounterVec,
    http_bytes_transmitted: CounterVec,
    http_calls_received: CounterVec,
    horizontal_adapter_resolve_time: HistogramVec,
    horizontal_adapter_resolved_promises: CounterVec,
    horizontal_adapter_uncomplete_promises: CounterVec,
    horizontal_adapter_sent_requests: CounterVec,
    horizontal_adapter_received_requests: CounterVec,
    horizontal_adapter_received_responses: CounterVec,
}

pub struct InfraMetadata {
    data: HashMap<String, Value>,
}

pub struct PrometheusMetricsDriver {
    pub(crate) metrics: PrometheusMetrics,
    pub(crate) infra_metadata: InfraMetadata,
    pub(crate) server: Weak<Server>,
}

pub struct NamespaceTags {
    app_id: String,
    data: HashMap<String, Value>,
}

impl PrometheusMetricsDriver {
    pub fn new(server: Weak<Server>) -> Self {
        let register = Registry::new();
        Self {
            server: Weak::new(),
            metrics: PrometheusMetrics {
                registry: register,
                connected_sockets: GaugeVec::new(
                    prometheus::Opts::new("connected_sockets", "Number of connected sockets"),
                    &["app_id"],
                )
                .unwrap(),
                new_connections_total: CounterVec::new(
                    prometheus::Opts::new(
                        "new_connections_total",
                        "Total number of new connections",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                new_disconnections_total: CounterVec::new(
                    prometheus::Opts::new(
                        "new_disconnections_total",
                        "Total number of new disconnections",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                socket_bytes_received: CounterVec::new(
                    prometheus::Opts::new(
                        "socket_bytes_received",
                        "Total number of bytes received from sockets",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                socket_bytes_transmitted: CounterVec::new(
                    prometheus::Opts::new(
                        "socket_bytes_transmitted",
                        "Total number of bytes transmitted to sockets",
                    ),
                    &["app_id", "port"],
                )
                .unwrap(),
                ws_messages_received: CounterVec::new(
                    prometheus::Opts::new(
                        "ws_messages_received",
                        "Total number of messages received from websockets",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                ws_messages_sent: CounterVec::new(
                    prometheus::Opts::new(
                        "ws_messages_sent",
                        "Total number of messages sent to websockets",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                http_bytes_received: CounterVec::new(
                    prometheus::Opts::new(
                        "http_bytes_received",
                        "Total number of bytes received from HTTP",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                http_bytes_transmitted: CounterVec::new(
                    prometheus::Opts::new(
                        "http_bytes_transmitted",
                        "Total number of bytes transmitted to HTTP",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                http_calls_received: CounterVec::new(
                    prometheus::Opts::new(
                        "http_calls_received",
                        "Total number of calls received from HTTP",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                horizontal_adapter_resolve_time: HistogramVec::new(
                    prometheus::HistogramOpts::new(
                        "horizontal_adapter_resolve_time",
                        "Time taken to resolve a promise in the horizontal adapter",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                horizontal_adapter_resolved_promises: CounterVec::new(
                    prometheus::Opts::new(
                        "horizontal_adapter_resolved_promises",
                        "Total number of resolved promises in the horizontal adapter",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                horizontal_adapter_uncomplete_promises: CounterVec::new(
                    prometheus::Opts::new(
                        "horizontal_adapter_uncomplete_promises",
                        "Total number of uncomplete promises in the horizontal adapter",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                horizontal_adapter_sent_requests: CounterVec::new(
                    prometheus::Opts::new(
                        "horizontal_adapter_sent_requests",
                        "Total number of requests sent to the horizontal adapter",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                horizontal_adapter_received_requests: CounterVec::new(
                    prometheus::Opts::new(
                        "horizontal_adapter_received_requests",
                        "Total number of requests received by the horizontal adapter",
                    ),
                    &["app_id"],
                )
                .unwrap(),
                horizontal_adapter_received_responses: CounterVec::new(
                    prometheus::Opts::new(
                        "horizontal_adapter_received_responses",
                        "Total number of responses received by the horizontal adapter",
                    ),
                    &["app_id"],
                )
                .unwrap(),
            },
            infra_metadata: InfraMetadata {
                data: Default::default(),
            },
        }
    }
    pub fn get_tags(&self, app_id: String) -> NamespaceTags {
        return NamespaceTags {
            app_id,
            data: self.infra_metadata.data.clone(),
        };
    }
}

impl MetricsTrait for PrometheusMetricsDriver {
    fn mark_new_connection(&mut self, ws: WebSocket) {
        let app_id = ws.app_key.unwrap();
        self.metrics
            .connected_sockets
            .with_label_values(&[&app_id.as_str()])
            .inc();
        self.metrics
            .new_connections_total
            .with_label_values(&[&app_id.as_str()])
            .inc();
    }

    fn mark_disconnection(&self, ws: WebSocket) {
        let app_id = ws.app_key.unwrap();
        self.metrics
            .connected_sockets
            .with_label_values(&[&app_id.as_str()])
            .dec();
        self.metrics
            .new_disconnections_total
            .with_label_values(&[&app_id.as_str()])
            .inc();
    }

    fn mark_api_message(&self, app_id: String, incoming_message: Value, sent_message: Value) {
        self.metrics
            .http_bytes_received
            .with_label_values(&[&app_id.as_str()])
            .inc_by(Utils::data_to_bytes(Vec::from([incoming_message])) as f64);
        self.metrics
            .http_bytes_transmitted
            .with_label_values(&[&app_id.as_str()])
            .inc_by(Utils::data_to_bytes(Vec::from([sent_message])) as f64);
        self.metrics
            .http_calls_received
            .with_label_values(&[&app_id.as_str()])
            .inc();
    }

    fn mark_ws_message_sent(&self, app_id: String, sent_message: Value) {
        self.metrics
            .socket_bytes_transmitted
            .with_label_values(&[&app_id.as_str(), "ws"])
            .inc_by(Utils::data_to_bytes(Vec::from([sent_message])) as f64);
        self.metrics
            .ws_messages_sent
            .with_label_values(&[&app_id.as_str()])
            .inc();
    }

    fn mark_ws_message_received(&self, app_id: String, message: Value) {
        self.metrics
            .socket_bytes_received
            .with_label_values(&[&app_id.as_str()])
            .inc_by(message.as_str().unwrap().as_bytes().len() as f64);
        self.metrics
            .ws_messages_received
            .with_label_values(&[&app_id.as_str()])
            .inc();
    }

    fn track_horizontal_adapter_resolve_time(&self, app_id: String, time: u64) {
        self.metrics
            .horizontal_adapter_resolve_time
            .with_label_values(&[&app_id.as_str()])
            .observe(time as f64);
    }

    fn track_horizontal_adapter_resolved_promises(&self, app_id: String, resolved: bool) {
        if resolved {
            self.metrics
                .horizontal_adapter_resolved_promises
                .with_label_values(&[&app_id.as_str()])
                .inc();
        } else {
            self.metrics
                .horizontal_adapter_uncomplete_promises
                .with_label_values(&[&app_id.as_str()])
                .inc();
        }
    }

    fn mark_horizontal_adapter_request_sent(&self, app_id: String) {
        self.metrics
            .horizontal_adapter_sent_requests
            .with_label_values(&[&app_id.as_str()])
            .inc();
    }

    fn mark_horizontal_adapter_request_received(&self, app_id: String) {
        self.metrics
            .horizontal_adapter_received_requests
            .with_label_values(&[&app_id.as_str()])
            .inc();
    }

    fn mark_horizontal_adapter_response_received(&self, app_id: String) {
        self.metrics
            .horizontal_adapter_received_responses
            .with_label_values(&[&app_id.as_str()])
            .inc();
    }

    async fn get_metrics_as_plaintext(&self) -> Result<String, ()> {
        let encoder = TextEncoder::new();
        let metric_families = self.metrics.registry.gather();

        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        let metrics_string = String::from_utf8(buffer).unwrap();

        Ok(metrics_string)
    }

    async fn get_metrics_as_json(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let encoder = TextEncoder::new();
        let metric_families = self.metrics.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        let metrics_string = String::from_utf8(buffer).unwrap();
        let metrics_json: Value = match serde_json::from_str(&metrics_string) {
            Ok(json) => json,
            Err(e) => {
                return Err(Box::new(e));
            }
        };
        Ok(serde_json::json!(metrics_json))
    }

    fn clear(&self) {
        self.metrics.registry.gather().clear();
    }
}
