use crate::{Key, PRIVATE_KEY_BYTES};

pub struct ServerConfig {
    pub protocol_id: u64,
    pub private_key: Key,
    pub keep_alive_send_rate: f64,
    pub client_timeout_secs: i32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            keep_alive_send_rate: 0.1,
            client_timeout_secs: 15,
            protocol_id: 0,
            private_key: [0; PRIVATE_KEY_BYTES],
        }
    }
}
