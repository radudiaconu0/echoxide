use crate::ws_handler::WebSocket;
use prometheus::proto::MetricFamily;
use serde_json::Value;

pub trait MetricsTrait {
    fn mark_new_connection(&mut self, ws: WebSocket);
    fn mark_disconnection(&self, ws: WebSocket);
    fn mark_api_message(&self, app_id: String, incoming_message: Value, sent_message: Value);
    fn mark_ws_message_sent(&self, app_id: String, sent_message: Value);
    fn mark_ws_message_received(&self, app_id: String, message: Value);
    fn track_horizontal_adapter_resolve_time(&self, app_id: String, time: u64);
    fn track_horizontal_adapter_resolved_promises(&self, app_id: String, resolved: bool);
    fn mark_horizontal_adapter_request_sent(&self, app_id: String);
    fn mark_horizontal_adapter_request_received(&self, app_id: String);
    fn mark_horizontal_adapter_response_received(&self, app_id: String);
    async fn get_metrics_as_plaintext(&self) -> Result<String, ()>;
    async fn get_metrics_as_json(&self) -> Result<Value, Box<dyn std::error::Error>>;
    fn clear(&self);
}
