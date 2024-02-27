use aws_sdk_lambda::Config;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct WebhookInterface {
    url: Option<String>,
    headers: Option<HashMap<String, String>>,
    lambda_function: Option<String>,
    event_types: Vec<String>,
    filter: Option<HashMap<String, String>>,
    // lambda: Option<LambdaOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LambdaOptions {
    async_: Option<bool>,
    region: Option<String>,
    // client_options: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct App {
    id: String,
    key: String,
    secret: String,
    max_connections: i64,
    enable_client_messages: bool,
    enabled: bool,
    max_backend_events_per_second: i64,
    max_client_events_per_second: i64,
    max_read_requests_per_minute: i64,
    webhooks: Vec<Value>,
    max_presence_member_size_in_kb: i64,
    max_channel_name_length: i64,
    max_event_channel_at_once: i64,
    max_event_name_length: i64,
    max_event_payload_in_kb: i64,
    max_event_batch_size: i64,
    enable_user_authentication: bool,
    has_client_event_webhooks: bool,
    has_channel_occupied_webhooks: bool,
    has_channel_vacated_webhooks: bool,
    has_member_added_webhooks: bool,
    has_member_removed_webhooks: bool,
    has_cache_missed_webhooks: bool,
}

impl App {
    fn new(initial_app: HashMap<&str, serde_json::Value>) -> Self {
        // Example of creating an App instance from a HashMap.
        // Actual implementation would parse and convert fields as needed.
        App {
            id: initial_app.get("id").unwrap().as_str().unwrap().to_string(),
            key: initial_app
                .get("key")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            secret: initial_app
                .get("secret")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            max_connections: initial_app
                .get("maxConnections")
                .unwrap()
                .as_i64()
                .unwrap_or(-1),
            enable_client_messages: initial_app
                .get("enableClientMessages")
                .unwrap()
                .as_bool()
                .unwrap_or(false),
            enabled: initial_app
                .get("enabled")
                .unwrap()
                .as_bool()
                .unwrap_or(true),
            max_backend_events_per_second: initial_app
                .get("maxBackendEventsPerSecond")
                .unwrap()
                .as_i64()
                .unwrap_or(-1),
            max_client_events_per_second: initial_app
                .get("maxClientEventsPerSecond")
                .unwrap()
                .as_i64()
                .unwrap_or(-1),
            max_read_requests_per_minute: initial_app
                .get("maxReadRequestsPerMinute")
                .unwrap()
                .as_i64()
                .unwrap_or(-1),
            webhooks: Self::transform_potential_json_to_array(initial_app.get("webhooks").unwrap()),
            max_presence_member_size_in_kb: 0,
            max_channel_name_length: 0,
            max_event_channel_at_once: 0,
            max_event_name_length: 0,
            max_event_payload_in_kb: 0,
            max_event_batch_size: 0,
            enable_user_authentication: false,
            has_client_event_webhooks: false,
            has_channel_occupied_webhooks: false,
            has_channel_vacated_webhooks: false,
            has_member_added_webhooks: false,
            has_member_removed_webhooks: false,
            has_cache_missed_webhooks: false,
        }
    }
    fn transform_potential_json_to_array(potential_json: &Value) -> Vec<Value> {
        match potential_json {
            Value::Array(arr) => arr.clone(),
            Value::String(s) => {
                match from_str::<Value>(s) {
                    Ok(Value::Array(arr)) => arr,
                    _ => vec![], // Not a JSON array or not parseable as JSON
                }
            }
            _ => vec![], // Not a string or array
        }
    }
}
