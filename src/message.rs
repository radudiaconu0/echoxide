use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// For fields that can be of any type, you might need a custom enum or use serde_json::Value
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AnyValue {
    String(String),
    Map(HashMap<String, serde_json::Value>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageData {
    pub channel_data: Option<String>,
    pub channel: Option<String>,
    pub user_data: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>, // For additional dynamic fields
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PusherMessage {
    pub channel: Option<String>,
    pub name: Option<String>,
    pub event: Option<String>,
    pub data: Option<MessageData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PusherApiMessage {
    pub name: Option<String>,
    pub data: Option<AnyValue>,
    pub channel: Option<String>,
    pub channels: Option<Vec<String>>,
    pub socket_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentPusherMessage {
    pub channel: Option<String>,
    pub event: Option<String>,
    pub data: Option<AnyValue>,
}

// For the uWebSocketMessage, Rust doesn't directly support union types like TypeScript.
// You might define an enum to represent the different possible types.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum UWebSocketMessage {
    ArrayBuffer(Vec<u8>),
    PusherMessage(PusherMessage),
}
