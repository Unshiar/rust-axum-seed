use std::net::{AddrParseError, Ipv4Addr};
use std::num::ParseIntError;

pub const ENV_DB_URL_NAME: &str = "DB_URL";
pub const DB_URL_DEFAULT: &str = "postgres://user:user@localhost:5432/db-test";
pub const ENV_HOST_NAME: &str = "HOST";
pub const HOST_DEFAULT: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
pub const ENV_PORT_NAME: &str = "PORT";
pub const PORT_DEFAULT: u16 = 8080;

pub fn get_env_db_url() -> String {
    std::env::var(ENV_DB_URL_NAME).unwrap_or_else(|_| {
        tracing::warn!("env '{ENV_DB_URL_NAME}' not set, using default");
        DB_URL_DEFAULT.to_string()
    })
}

pub fn get_env_host() -> Result<Ipv4Addr, AddrParseError> {
    std::env::var(ENV_HOST_NAME)
        .unwrap_or_else(|_| {
            tracing::warn!("env '{ENV_HOST_NAME}' not set, using default");
            HOST_DEFAULT.to_string()
        })
        .parse::<Ipv4Addr>()
        .inspect_err(|er| {
            tracing::error!("env '{ENV_HOST_NAME}' should be IPv4 format: {}", er);
        })
}

pub fn get_env_port() -> Result<u16, ParseIntError> {
    std::env::var(ENV_PORT_NAME)
        .unwrap_or_else(|_| {
            tracing::warn!("env '{ENV_PORT_NAME}' not set, using default");
            PORT_DEFAULT.to_string()
        })
        .parse::<u16>()
        .inspect_err(|er| {
            tracing::error!(
                "env '{ENV_PORT_NAME}' should be in range [0, 65535]: {}",
                er
            );
        })
}
