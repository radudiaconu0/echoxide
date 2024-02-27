use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PresenceMemberInfo {
    pub data: HashMap<String, Value>,
}

pub struct PresenceMember {
    pub(crate) user_id: String,
    pub(crate) user_info: PresenceMemberInfo,
    pub(crate) socket_id: Option<String>,
}
