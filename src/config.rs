use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_environment")]
    pub environment: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_environment() -> String {
    "development".to_string()
}

impl AppConfig {
    pub fn from_env() -> Self {
        // Load .env file
        dotenvy::dotenv().ok();

        // Load config
        let cfg = config::Config::builder()
            .add_source(config::Environment::default()) // read from ENV
            .build()
            .unwrap();

        cfg.try_deserialize::<AppConfig>().unwrap()
    }

    /// Get the server address as a SocketAddr
    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid server address")
    }

    /// Check if running in production environment
    pub fn is_production(&self) -> bool {
        self.environment.eq_ignore_ascii_case("production")
    }

    /// Check if running in development environment
    pub fn is_development(&self) -> bool {
        self.environment.eq_ignore_ascii_case("development")
    }
}
