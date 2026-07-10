pub mod database;
pub mod errors;
pub mod handlers;
pub mod log;
pub mod misc;

#[cfg(test)]
mod tests {
    use super::misc::env_handle::{
        get_env_db_url, get_env_host, get_env_port, DB_URL_DEFAULT, ENV_DB_URL_NAME, ENV_HOST_NAME,
        ENV_PORT_NAME, HOST_DEFAULT, PORT_DEFAULT,
    };
    use serial_test::serial;

    #[serial]
    #[test]
    fn test_get_env_db_url_default() {
        std::env::remove_var(ENV_DB_URL_NAME);
        let url = get_env_db_url();
        assert_eq!(url, DB_URL_DEFAULT);
    }

    #[serial]
    #[test]
    fn test_get_env_db_url_custom() {
        let expected = "postgres://test:test@localhost:5432/testdb";
        std::env::set_var(ENV_DB_URL_NAME, expected);
        let url = get_env_db_url();
        assert_eq!(url, expected);
    }

    #[serial]
    #[test]
    fn test_get_env_host_default() {
        std::env::remove_var(ENV_HOST_NAME);
        let host = get_env_host().unwrap();
        assert_eq!(host.to_string(), HOST_DEFAULT.to_string());
    }

    #[serial]
    #[test]
    fn test_get_env_host_custom() {
        std::env::set_var(ENV_HOST_NAME, "0.0.0.0");
        let host = get_env_host().unwrap();
        assert_eq!(host.to_string(), "0.0.0.0");
    }

    #[serial]
    #[test]
    fn test_get_env_host_invalid() {
        std::env::set_var(ENV_HOST_NAME, "invalid-host");
        let result = get_env_host();
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_get_env_host_invalid_ipv4_format() {
        std::env::set_var(ENV_HOST_NAME, "127.0.0.777");
        let result = get_env_host();
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_get_env_port_default() {
        std::env::remove_var(ENV_PORT_NAME);
        let port = get_env_port().unwrap();
        assert_eq!(port, PORT_DEFAULT);
    }

    #[serial]
    #[test]
    fn test_get_env_port_custom() {
        std::env::set_var(ENV_PORT_NAME, "8080");
        let port = get_env_port().unwrap();
        assert_eq!(port, 8080);
    }

    #[serial]
    #[test]
    fn test_get_env_port_invalid() {
        std::env::set_var(ENV_PORT_NAME, "invalid-port");
        let result = get_env_port();
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_get_env_port_invalid_negative_value() {
        std::env::set_var(ENV_PORT_NAME, "-1111");
        let result = get_env_port();
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_get_env_port_invalid_positive_value() {
        std::env::set_var(ENV_PORT_NAME, "65 536");
        let result = get_env_port();
        assert!(result.is_err());
    }
}
