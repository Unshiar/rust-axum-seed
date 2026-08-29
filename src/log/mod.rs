use std::fs;
use std::sync::Arc;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer, Registry};

const LOG_FILE: &str = "/var/log/app.log";

struct LogConfig {
    enable_file: bool,
}

pub fn init_logging() {
    let config = LogConfig { enable_file: true };

    let env_filter =
        EnvFilter::new("trace,entities=trace,sea_orm_migration=info,sqlx=warn,sea_orm=warn");

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_filter(env_filter.clone())
        .boxed();

    let subscriber = Registry::default().with(stdout_layer);

    if config.enable_file {
        match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_FILE)
        {
            Ok(file) => {
                let file_layer = tracing_subscriber::fmt::layer()
                    .with_target(true)
                    .with_writer(Arc::new(file))
                    .with_filter(env_filter);

                subscriber.with(file_layer).init();
                return;
            }
            Err(err) => {
                subscriber.init();

                tracing::error!(
                    "Can't open log file '{}': {}. Logging will be done only to stdout.",
                    LOG_FILE,
                    err
                );
                return;
            }
        }
    }

    subscriber.init();
}
