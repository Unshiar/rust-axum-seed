use std::net::Ipv4Addr;

pub const ENV_DB_URL_NAME: &str = "DB_URL";
pub const DB_URL_DEFAULT: &str = "postgres://user:user@localhost:5432/db-test";
pub const ENV_HOST_NAME: &str = "HOST";
pub const HOST_DEFAULT: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
pub const ENV_PORT_NAME: &str = "PORT";
pub const PORT_DEFAULT: u16 = 8080;
