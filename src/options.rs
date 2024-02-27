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

struct ClusterNode {
    pub host: String,
    pub port: u16,
}

struct RedisSentinel {
    host: String,
    port: u16,
}

pub struct Adapter {
    pub driver: String,
    redis: RedisAdapter,
    cluster: ClusterAdapter,
    nats: NatsAdapter,
}

pub struct RedisAdapter {
    pub request_timeout: i64,
    prefix: String,
    redis_pub_options: Value,
    redis_sub_options: Value,
    cluster_mode: bool,
}

pub struct ClusterAdapter {
    pub request_timeout: i64,
}

pub struct NatsAdapter {
    pub request_timeout: i64,
    prefix: String,
    servers: Vec<String>,
    user: Option<String>,
    password: Option<String>,
    token: Option<String>,
    timeout: i64,
    nodes_number: Option<i64>,
}

pub struct AppManager {
    driver: String,
    array: ArrayAppManager,
    cache: CacheAppManager,
    mysql: MySQLAppManager,
}

pub struct ArrayAppManager {
    apps: Vec<App>,
}

struct CacheAppManager {
    enabled: bool,
    ttl: i64,
}

struct MySQLAppManager {
    table: String,
    version: String,
}

struct Options {
    adapter: Adapter,
    app_manager: AppManager,
}
