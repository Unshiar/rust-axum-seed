use std::fs;
use std::sync::Arc;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

const LOG_FILE: &str = "/var/log/app.log";

struct LogConfig {
    enable_file: bool,
}

pub fn init_logging() {
    let config = LogConfig { enable_file: true };

    let env_filter =
        EnvFilter::new("axum_app=info,entities=info,sea_orm_migration=info,sqlx=warn,sea_orm=warn,tower_http=debug");

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
            .open(LOG_FILE)
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
            LOG_FILE,
            err
        );
    }
}
