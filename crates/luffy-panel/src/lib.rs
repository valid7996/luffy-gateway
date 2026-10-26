//! Luffy Panel: Web API server, authentication, SQLite state, and subscription handler.

pub struct PanelConfig {
    pub port: u16,
    pub host: String,
}

impl Default for PanelConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "0.0.0.0".to_string(),
        }
    }
}
