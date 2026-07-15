use crate::misc::env_handle::{build_postgres_db_url, build_socket_addr};
use std::net::SocketAddr;

pub struct DbConfig {
    db_url: String,
}

pub struct ServerConfig {
    addr: SocketAddr,
}

pub struct AppConfig {
    pub db: DbConfig,
    pub server: ServerConfig,
}

impl DbConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = build_postgres_db_url()?;
        Ok(Self { db_url })
    }

    pub fn db_url(&self) -> String {
        self.db_url.clone()
    }
}

impl ServerConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let addr = build_socket_addr()?;
        Ok(Self { addr })
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self {
            db: DbConfig::from_env()?,
            server: ServerConfig::from_env()?,
        };
        Ok(config)
    }
}
