use crate::misc::env_handle::{build_postgres_db_url, build_socket_addr};
use std::net::SocketAddr;

/// Database configuration loaded from environment variables.
///
/// Holds the connection URL for PostgreSQL/SQLite connections.
/// Configuration is loaded from the environment at startup.
pub struct DbConfig {
    db_url: String,
}

/// Server configuration loaded from environment variables.
///
/// Contains the socket address (host and port) where the server will listen.
/// Configuration is loaded from the environment at startup.
pub struct ServerConfig {
    addr: SocketAddr,
}

/// Application configuration combining database and server settings.
///
/// Provides a single point to access all application-level configuration
/// loaded from environment variables with sensible defaults.
pub struct AppConfig {
    pub db: DbConfig,
    pub server: ServerConfig,
}

impl DbConfig {
    /// Create database configuration from environment variables.
    ///
    /// Reads database connection parameters (host, port, user, password, database name)
    /// from environment variables and constructs a PostgreSQL connection URL.
    ///
    /// # Returns
    ///
    /// `Ok(DbConfig)` with the parsed configuration, or `Err` if parsing fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let db_config = DbConfig::from_env()?;
    /// let url = db_config.db_url();
    /// ```
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = build_postgres_db_url()?;
        Ok(Self { db_url })
    }

    /// Get the database connection URL.
    ///
    /// Returns a PostgreSQL connection string in format:
    /// `postgres://user:password@host:port/database`
    pub fn db_url(&self) -> String {
        self.db_url.clone()
    }
}

impl ServerConfig {
    /// Create server configuration from environment variables.
    ///
    /// Reads server bind address (HOST and PORT) from environment variables
    /// and constructs a socket address for the server to listen on.
    ///
    /// # Returns
    ///
    /// `Ok(ServerConfig)` with the parsed configuration, or `Err` if parsing fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let server_config = ServerConfig::from_env()?;
    /// let listener = TcpListener::bind(server_config.addr()).await?;
    /// ```
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let addr = build_socket_addr()?;
        Ok(Self { addr })
    }

    /// Get the server socket address.
    ///
    /// Returns the IPv4 address and port where the server should listen.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

impl AppConfig {
    /// Create full application configuration from environment variables.
    ///
    /// Loads both database and server configuration from environment variables.
    /// This is typically called once at application startup before creating the server.
    ///
    /// # Returns
    ///
    /// `Ok(AppConfig)` with all configuration loaded, or `Err` if any parsing fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let app_config = AppConfig::from_env()?;
    /// println!("Server will listen on: {}", app_config.server.addr());
    /// ```
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Self {
            db: DbConfig::from_env()?,
            server: ServerConfig::from_env()?,
        };
        Ok(config)
    }
}
