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
    #[serde(default = "default_database_url")]
    pub database_url: String,
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

fn default_database_url() -> String {
    "postgres://postgres:pgadmin@localhost:5432/postgres".to_string()
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        // Use "__" separator so DATABASE_URL maps to database_url (not database.url)
        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()
            .unwrap();

        cfg.try_deserialize::<AppConfig>().unwrap()
    }

    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid server address")
    }

    pub fn is_production(&self) -> bool {
        self.environment.eq_ignore_ascii_case("production")
    }

    #[allow(dead_code)]
    pub fn is_development(&self) -> bool {
        self.environment.eq_ignore_ascii_case("development")
    }
}
