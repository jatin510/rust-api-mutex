use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
    // .expect("Failed to setup settings");
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: Server,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            server: Server { port: 3050 },
        }
    }
}
