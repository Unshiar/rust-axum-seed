use tokio::signal;

pub async fn shutdown_signal() {
    signal::ctrl_c().await.expect("failed to listen for event");
    tracing::info!("Received Ctrl+C signal, shutting down...");
}
