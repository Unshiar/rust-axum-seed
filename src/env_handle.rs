use crate::constants::{
    DB_URL_DEFAULT, ENV_DB_URL_NAME, ENV_HOST_NAME, ENV_PORT_NAME, HOST_DEFAULT, PORT_DEFAULT,
};
use std::net::{AddrParseError, Ipv4Addr};
use std::num::ParseIntError;

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
