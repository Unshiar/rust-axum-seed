use crate::constants::{
    DB_URL_DEFAULT, ENV_DB_URL_NAME, ENV_HOST_NAME, ENV_PORT_NAME, HOST_DEFAULT, PORT_DEFAULT,
};
use std::net::Ipv4Addr;

pub fn get_env_db_url() -> String {
    std::env::var(ENV_DB_URL_NAME).unwrap_or_else(|_| {
        tracing::warn!("env '{ENV_DB_URL_NAME}' not set, using default");
        DB_URL_DEFAULT.to_string()
    })
}

pub fn get_env_host() -> Ipv4Addr {
    std::env::var(ENV_HOST_NAME)
        .unwrap_or_else(|_| {
            tracing::warn!("env '{ENV_HOST_NAME}' not set, using default");
            HOST_DEFAULT.to_string()
        })
        .parse::<Ipv4Addr>()
        .expect("env HOST should be IPv4")
}

pub fn get_env_port() -> u16 {
    std::env::var(ENV_PORT_NAME)
        .unwrap_or_else(|_| {
            tracing::warn!("env '{ENV_PORT_NAME}' not set, using default");
            PORT_DEFAULT.to_string()
        })
        .parse::<u16>()
        .expect("env PORT should be in range [0, 65535]")
}
