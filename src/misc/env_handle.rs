use std::net::{AddrParseError, Ipv4Addr, SocketAddr};
use std::num::ParseIntError;

pub const ENV_DB_USER_NAME: &str = "DATABASE_USER";
pub const DB_USER_DEFAULT: &str = "user";
pub const ENV_DB_PASSWORD_NAME: &str = "DATABASE_PASSWORD";
pub const DB_PASSWORD_DEFAULT: &str = "user";
pub const ENV_DB_NAME_NAME: &str = "DATABASE_NAME";
pub const DB_NAME_DEFAULT: &str = "db-test";
pub const ENV_DB_HOST_NAME: &str = "DATABASE_HOST";
pub const DB_HOST_DEFAULT: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
pub const ENV_DB_PORT_NAME: &str = "DATABASE_PORT";
pub const DB_PORT_DEFAULT: u16 = 5432;
pub const ENV_HOST_NAME: &str = "HOST";
pub const HOST_DEFAULT: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
pub const ENV_PORT_NAME: &str = "PORT";
pub const PORT_DEFAULT: u16 = 8080;

fn get_str_env_by_name(env_name: &str, default_value: &str) -> String {
    std::env::var(env_name).unwrap_or_else(|_| {
        tracing::warn!("env '{env_name}' is not set, using default");
        default_value.to_string()
    })
}

pub fn build_postgres_db_url() -> Result<String, Box<dyn std::error::Error>> {
    let database_host = get_env_host_by_name(ENV_DB_HOST_NAME, DB_HOST_DEFAULT)?;
    let database_port = get_env_port_by_name(ENV_DB_PORT_NAME, DB_PORT_DEFAULT)?;
    Ok(format!(
        "postgres://{}:{}@{}:{}/{}",
        get_str_env_by_name(ENV_DB_USER_NAME, DB_USER_DEFAULT),
        get_str_env_by_name(ENV_DB_PASSWORD_NAME, DB_PASSWORD_DEFAULT),
        database_host,
        database_port,
        get_str_env_by_name(ENV_DB_NAME_NAME, DB_NAME_DEFAULT)
    ))
}

pub fn get_env_host_by_name(
    env_name: &str,
    default_value: Ipv4Addr,
) -> Result<Ipv4Addr, AddrParseError> {
    let host_str = get_str_env_by_name(env_name, &default_value.to_string());

    host_str.parse::<Ipv4Addr>().inspect_err(|err| {
        tracing::error!("env '{env_name}' should be IPv4 format: {}", err);
    })
}

pub fn get_env_port_by_name(env_name: &str, default_value: u16) -> Result<u16, ParseIntError> {
    let port_str = get_str_env_by_name(env_name, &default_value.to_string());

    port_str.parse::<u16>().inspect_err(|err| {
        tracing::error!("env '{env_name}' should be in range [0, 65535]: {}", err);
    })
}

pub fn build_socket_addr() -> Result<SocketAddr, Box<dyn std::error::Error>> {
    let host = get_env_host_by_name(ENV_HOST_NAME, HOST_DEFAULT)?;
    let port = get_env_port_by_name(ENV_PORT_NAME, PORT_DEFAULT)?;
    Ok(SocketAddr::from((host, port)))
}
