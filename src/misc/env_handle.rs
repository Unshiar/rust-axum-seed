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

/// Retrieve a string environment variable with fallback to default value.
///
/// Logs a warning if the environment variable is not set.
///
/// # Arguments
///
/// * `env_name` - Name of the environment variable to read
/// * `default_value` - Default value to use if variable is not set
///
/// # Returns
///
/// Either the value from the environment variable or the default value
fn get_str_env_by_name(env_name: &str, default_value: &str) -> String {
    std::env::var(env_name).unwrap_or_else(|_| {
        tracing::warn!("env '{env_name}' is not set, using default");
        default_value.to_string()
    })
}

/// Build a PostgreSQL connection URL from environment variables.
///
/// Constructs a connection string using environment variables with sensible defaults.
/// Logs warnings for any missing environment variables.
///
/// # Returns
///
/// `Ok(url_string)` with format `postgres://user:password@host:port/database`
/// or `Err` if host/port parsing fails
///
/// # Example
///
/// ```ignore
/// let db_url = build_postgres_db_url()?;
/// println!("Connecting to: {}", db_url);
/// ```
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

/// Parse an IPv4 address from an environment variable.
///
/// Reads the environment variable and parses it as an IPv4 address.
/// Logs an error if parsing fails. Returns the default value if variable is not set.
///
/// # Arguments
///
/// * `env_name` - Name of the environment variable to read
/// * `default_value` - Default IPv4 address to use if variable is not set
///
/// # Returns
///
/// `Ok(Ipv4Addr)` if parsing succeeds, or `Err` if the value is not a valid IPv4 address
pub fn get_env_host_by_name(
    env_name: &str,
    default_value: Ipv4Addr,
) -> Result<Ipv4Addr, AddrParseError> {
    let host_str = get_str_env_by_name(env_name, &default_value.to_string());

    host_str.parse::<Ipv4Addr>().inspect_err(|err| {
        tracing::error!("env '{env_name}' should be IPv4 format: {}", err);
    })
}

/// Parse a port number from an environment variable.
///
/// Reads the environment variable and parses it as a u16 (port number in range 0-65535).
/// Logs an error if parsing fails. Returns the default value if variable is not set.
///
/// # Arguments
///
/// * `env_name` - Name of the environment variable to read
/// * `default_value` - Default port number to use if variable is not set
///
/// # Returns
///
/// `Ok(port)` if parsing succeeds, or `Err` if the value is not a valid port number
pub fn get_env_port_by_name(env_name: &str, default_value: u16) -> Result<u16, ParseIntError> {
    let port_str = get_str_env_by_name(env_name, &default_value.to_string());

    port_str.parse::<u16>().inspect_err(|err| {
        tracing::error!("env '{env_name}' should be in range [0, 65535]: {}", err);
    })
}

/// Build a socket address (host:port) from environment variables.
///
/// Constructs a `SocketAddr` by reading HOST and PORT environment variables.
/// Uses default values if variables are not set.
///
/// # Returns
///
/// `Ok(SocketAddr)` with the server bind address, or `Err` if host/port parsing fails
///
/// # Example
///
/// ```ignore
/// let addr = build_socket_addr()?;
/// println!("Server will listen on: {}", addr);
/// ```
pub fn build_socket_addr() -> Result<SocketAddr, Box<dyn std::error::Error>> {
    let host = get_env_host_by_name(ENV_HOST_NAME, HOST_DEFAULT)?;
    let port = get_env_port_by_name(ENV_PORT_NAME, PORT_DEFAULT)?;
    Ok(SocketAddr::from((host, port)))
}
