use serde::Deserialize;
pub struct Daemon {
}

#[derive(Debug, Deserialize)]
pub struct DaemonConfig {
    /// cryptographic seed for this daemon instance
    // seed: Option<String>,
    /// HTTP host
    pub host: String,
    /// HTTP port
    pub port: u16,
}

