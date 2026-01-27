use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub http_addr: String,
    pub http_port: u16,
    pub tick_rate: f32,
    pub input_queue_limit: usize,
    pub input_rate_limit_per_sec: u32,
    pub max_message_bytes: usize,
    pub session_claims_per_minute: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            http_addr: "0.0.0.0".to_string(),
            http_port: 8080,
            tick_rate: 20.0,
            input_queue_limit: 1024,
            input_rate_limit_per_sec: 30,
            max_message_bytes: 16 * 1024,
            session_claims_per_minute: 10,
        }
    }
}
