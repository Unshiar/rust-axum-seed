pub mod schemas;
pub mod database;
pub mod errors;
pub mod handlers;
pub mod log;
pub mod misc;

#[cfg(test)]
mod tests {
    use super::misc::env_handle::{
        build_postgres_db_url, get_env_host_by_name, get_env_port_by_name, DB_HOST_DEFAULT,
        DB_NAME_DEFAULT, DB_PASSWORD_DEFAULT, DB_PORT_DEFAULT, DB_USER_DEFAULT, ENV_DB_HOST_NAME,
        ENV_DB_NAME_NAME, ENV_DB_PASSWORD_NAME, ENV_DB_PORT_NAME, ENV_DB_USER_NAME, ENV_HOST_NAME,
        ENV_PORT_NAME, HOST_DEFAULT, PORT_DEFAULT,
    };
    use serial_test::serial;

    #[serial]
    #[test]
    fn test_build_postgres_db_url_default() {
        std::env::remove_var(ENV_DB_USER_NAME);
        std::env::remove_var(ENV_DB_PASSWORD_NAME);
        std::env::remove_var(ENV_DB_HOST_NAME);
        std::env::remove_var(ENV_DB_PORT_NAME);
        std::env::remove_var(ENV_DB_NAME_NAME);
        let expected = format!(
            "postgres://{}:{}@{}:{}/{}",
            DB_USER_DEFAULT, DB_PASSWORD_DEFAULT, DB_HOST_DEFAULT, DB_PORT_DEFAULT, DB_NAME_DEFAULT
        );
        let result = build_postgres_db_url();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[serial]
    #[test]
    fn test_build_postgres_db_url_custom() {
        std::env::set_var(ENV_DB_USER_NAME, "db_user");
        std::env::set_var(ENV_DB_PASSWORD_NAME, "db_user_password");
        std::env::set_var(ENV_DB_HOST_NAME, "127.0.0.1");
        std::env::set_var(ENV_DB_PORT_NAME, "5432");
        std::env::set_var(ENV_DB_NAME_NAME, "db_name");
        let expected = format!(
            "postgres://{}:{}@{}:{}/{}",
            "db_user", "db_user_password", "127.0.0.1", "5432", "db_name"
        );
        let result = build_postgres_db_url();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[serial]
    #[test]
    fn test_build_postgres_db_url_custom_with_user_name_default() {
        std::env::remove_var(ENV_DB_USER_NAME);
        std::env::set_var(ENV_DB_PASSWORD_NAME, "db_user_password");
        std::env::set_var(ENV_DB_HOST_NAME, "127.0.0.1");
        std::env::set_var(ENV_DB_PORT_NAME, "5432");
        std::env::set_var(ENV_DB_NAME_NAME, "db_name");
        let expected = format!(
            "postgres://{}:{}@{}:{}/{}",
            DB_USER_DEFAULT, "db_user_password", "127.0.0.1", "5432", "db_name"
        );
        let result = build_postgres_db_url();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[serial]
    #[test]
    fn test_build_postgres_db_url_custom_with_user_password_default() {
        std::env::set_var(ENV_DB_USER_NAME, "db_user");
        std::env::remove_var(ENV_DB_PASSWORD_NAME);
        std::env::set_var(ENV_DB_HOST_NAME, "127.0.0.1");
        std::env::set_var(ENV_DB_PORT_NAME, "5432");
        std::env::set_var(ENV_DB_NAME_NAME, "db_name");
        let expected = format!(
            "postgres://{}:{}@{}:{}/{}",
            "db_user", DB_PASSWORD_DEFAULT, "127.0.0.1", "5432", "db_name"
        );
        let result = build_postgres_db_url();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[serial]
    #[test]
    fn test_build_postgres_db_url_custom_with_db_host_default() {
        std::env::set_var(ENV_DB_USER_NAME, "db_user");
        std::env::set_var(ENV_DB_PASSWORD_NAME, "db_user_password");
        std::env::remove_var(ENV_DB_HOST_NAME);
        std::env::set_var(ENV_DB_PORT_NAME, "5432");
        std::env::set_var(ENV_DB_NAME_NAME, "db_name");
        let expected = format!(
            "postgres://{}:{}@{}:{}/{}",
            "db_user", "db_user_password", DB_HOST_DEFAULT, "5432", "db_name"
        );
        let result = build_postgres_db_url();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[serial]
    #[test]
    fn test_build_postgres_db_url_custom_with_db_port_default() {
        std::env::set_var(ENV_DB_USER_NAME, "db_user");
        std::env::set_var(ENV_DB_PASSWORD_NAME, "db_user_password");
        std::env::set_var(ENV_DB_HOST_NAME, "127.0.0.1");
        std::env::remove_var(ENV_DB_PORT_NAME);
        std::env::set_var(ENV_DB_NAME_NAME, "db_name");
        let expected = format!(
            "postgres://{}:{}@{}:{}/{}",
            "db_user", "db_user_password", "127.0.0.1", DB_PORT_DEFAULT, "db_name"
        );
        let result = build_postgres_db_url();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[serial]
    #[test]
    fn test_build_postgres_db_url_custom_with_db_name_default() {
        std::env::set_var(ENV_DB_USER_NAME, "db_user");
        std::env::set_var(ENV_DB_PASSWORD_NAME, "db_user_password");
        std::env::set_var(ENV_DB_HOST_NAME, "127.0.0.1");
        std::env::set_var(ENV_DB_PORT_NAME, "5432");
        std::env::remove_var(ENV_DB_NAME_NAME);
        let expected = format!(
            "postgres://{}:{}@{}:{}/{}",
            "db_user", "db_user_password", "127.0.0.1", "5432", DB_NAME_DEFAULT
        );
        let result = build_postgres_db_url();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[serial]
    #[test]
    fn test_build_postgres_db_url_non_ipv4_format_db_host() {
        std::env::set_var(ENV_DB_USER_NAME, "db_user");
        std::env::set_var(ENV_DB_PASSWORD_NAME, "db_user_password");
        std::env::set_var(ENV_DB_HOST_NAME, "non_ipv4_format_host");
        std::env::set_var(ENV_DB_PORT_NAME, "5432");
        std::env::remove_var(ENV_DB_NAME_NAME);
        let result = build_postgres_db_url();
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_build_postgres_db_url_wrong_ipv4_format_db_host() {
        std::env::set_var(ENV_DB_USER_NAME, "db_user");
        std::env::set_var(ENV_DB_PASSWORD_NAME, "db_user_password");
        std::env::set_var(ENV_DB_HOST_NAME, "127.0.0.999");
        std::env::set_var(ENV_DB_PORT_NAME, "5432");
        std::env::remove_var(ENV_DB_NAME_NAME);
        let result = build_postgres_db_url();
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_build_postgres_db_url_invalid_format_db_port() {
        std::env::set_var(ENV_DB_USER_NAME, "db_user");
        std::env::set_var(ENV_DB_PASSWORD_NAME, "db_user_password");
        std::env::set_var(ENV_DB_HOST_NAME, "127.0.0.1");
        std::env::set_var(ENV_DB_PORT_NAME, "invalid_port_format");
        std::env::remove_var(ENV_DB_NAME_NAME);
        let result = build_postgres_db_url();
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_get_env_host_default() {
        std::env::remove_var(ENV_HOST_NAME);
        let result = get_env_host_by_name(ENV_HOST_NAME, HOST_DEFAULT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), HOST_DEFAULT.to_string());
    }

    #[serial]
    #[test]
    fn test_get_env_host_custom() {
        std::env::set_var(ENV_HOST_NAME, "0.0.0.0");
        let result = get_env_host_by_name(ENV_HOST_NAME, HOST_DEFAULT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "0.0.0.0");
    }

    #[serial]
    #[test]
    fn test_get_env_host_invalid() {
        std::env::set_var(ENV_HOST_NAME, "invalid-host");
        let result = get_env_host_by_name(ENV_HOST_NAME, HOST_DEFAULT);
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_get_env_host_invalid_ipv4_format() {
        std::env::set_var(ENV_HOST_NAME, "127.0.0.777");
        let result = get_env_host_by_name(ENV_HOST_NAME, HOST_DEFAULT);
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_get_env_port_default() {
        std::env::remove_var(ENV_PORT_NAME);
        let result = get_env_port_by_name(ENV_PORT_NAME, PORT_DEFAULT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PORT_DEFAULT);
    }

    #[serial]
    #[test]
    fn test_get_env_port_custom() {
        std::env::set_var(ENV_PORT_NAME, "8080");
        let result = get_env_port_by_name(ENV_PORT_NAME, PORT_DEFAULT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8080);
    }

    #[serial]
    #[test]
    fn test_get_env_port_invalid() {
        std::env::set_var(ENV_PORT_NAME, "invalid-port");
        let result = get_env_port_by_name(ENV_PORT_NAME, PORT_DEFAULT);
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_get_env_port_invalid_negative_value() {
        std::env::set_var(ENV_PORT_NAME, "-1111");
        let result = get_env_port_by_name(ENV_PORT_NAME, PORT_DEFAULT);
        assert!(result.is_err());
    }

    #[serial]
    #[test]
    fn test_get_env_port_invalid_positive_value() {
        std::env::set_var(ENV_PORT_NAME, "65 536");
        let result = get_env_port_by_name(ENV_PORT_NAME, PORT_DEFAULT);
        assert!(result.is_err());
    }
}
