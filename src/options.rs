use crate::app::App;
use serde_json::Value;

pub struct Redis {
    pub host: String,
    pub port: u16,
    pub db: i64,
    pub username: Option<String>,
    pub key_prefix: String,
    pub name: String,
    pub sentinels: Option<Vec<RedisSentinel>>,
    pub cluster: Option<Vec<ClusterNode>>,
}

pub struct ClusterNode {
    pub host: String,
    pub port: u16,
}

pub struct RedisSentinel {
    host: String,
    port: u16,
}

pub struct Adapter {
    pub driver: String,
    pub(crate) redis: RedisAdapter,
    pub(crate) cluster: ClusterAdapter,
    pub(crate) nats: NatsAdapter,
}

pub struct RedisAdapter {
    pub request_timeout: i64,
    pub(crate) prefix: String,
    pub(crate) redis_pub_options: Value,
    pub(crate) redis_sub_options: Value,
    pub(crate) cluster_mode: bool,
}

pub struct ClusterAdapter {
    pub request_timeout: i64,
}

pub struct NatsAdapter {
    pub request_timeout: i64,
    pub(crate) prefix: String,
    pub(crate) servers: Vec<String>,
    pub(crate) user: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) token: Option<String>,
    pub(crate) timeout: i64,
    pub(crate) nodes_number: Option<i64>,
}

pub struct AppManager {
    pub(crate) driver: String,
    pub(crate) array: ArrayAppManager,
    pub(crate) cache: CacheAppManager,
    pub(crate) mysql: MySQLAppManager,
}

pub struct ArrayAppManager {
    pub(crate) apps: Vec<App>,
}

pub struct CacheAppManager {
    pub(crate) enabled: bool,
    pub(crate) ttl: i64,
}

pub struct MySQLAppManager {
    pub(crate) table: String,
    pub(crate) version: String,
}

pub struct Prometheus {
    pub(crate) prefix: String,
}

pub struct Metrics {
    pub(crate) enabled: bool,
    pub(crate) driver: String,
    pub(crate) host: String,
    pub(crate) prometheus: Prometheus,
    pub(crate) port: u16,
}

pub struct Options {
    pub(crate) adapter: Adapter,
    pub(crate) app_manager: AppManager,
    pub(crate) debug: bool,
    pub(crate) port: u16,
    pub(crate) metrics: Metrics,
}
