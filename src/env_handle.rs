use crate::constants::{DB_URL, HOST, PORT};
use std::net::Ipv4Addr;

pub fn get_env_db_url() -> String {
    std::env::var("DB_URL").unwrap_or_else(|_| {
        tracing::warn!("env DB_URL not set, using default");
        DB_URL.to_string()
    })
}

pub fn get_env_host() -> Ipv4Addr {
    std::env::var("HOST")
        .unwrap_or_else(|_| {
            tracing::warn!("env HOST not set, using default");
            HOST.to_string()
        })
        .parse::<Ipv4Addr>()
        .expect("env HOST should be IPv4")
}

pub fn get_env_port() -> u16 {
    std::env::var("PORT")
        .unwrap_or_else(|_| {
            tracing::warn!("env PORT not set, using default");
            PORT.to_string()
        })
        .parse::<u16>()
        .expect("env PORT should be in range [0, 65535]")
}
