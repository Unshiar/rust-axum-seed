pub mod config;

use crate::log::config::LogConfig;
use std::fs;
use std::sync::Arc;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

const ENV_LOG_LEVEL: &str = "LOG_LEVEL";

fn get_log_level() -> String {
    std::env::var(ENV_LOG_LEVEL)
        .ok()
        .and_then(|level| level.parse::<tracing::Level>().ok())
        .unwrap_or(tracing::Level::INFO)
        .to_string()
}

pub fn init_logging(config: LogConfig) {
    let log_level = get_log_level();

    // Perhaps you will want to hardcode the logging level for a specific module
    let env_filter = EnvFilter::new(format!(
        "axum={},\
        generate_schema={},\
        axum_app={},\
        entities={},\
        sea_orm_migration={},\
        sqlx={},\
        sea_orm={},\
        tower_http={}",
        log_level, log_level, log_level, log_level, log_level, log_level, log_level, log_level
    ));

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        // Optional thread name
        // .with_thread_names(true)
        .with_thread_ids(true);

    let mut file_layer = None;
    let mut file_error = None;

    if config.enable_file {
        match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config.log_file)
        {
            Ok(file) => {
                file_layer = Some(
                    tracing_subscriber::fmt::layer()
                        .with_target(true)
                        .with_writer(Arc::new(file)),
                );
            }
            Err(err) => {
                file_error = Some(err);
            }
        }
    }

    Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .with(env_filter)
        .init();

    if let Some(err) = file_error {
        tracing::error!(
            "Can't open log file '{}': {}. Logging will be done only to stdout.",
            config.log_file,
            err
        );
    }
}
