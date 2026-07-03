pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            "trace,entities=trace,sea_orm_migration=info,sqlx=warn,sea_orm=warn",
        ))
        .init();
}
