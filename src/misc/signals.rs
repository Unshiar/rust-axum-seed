use tokio::{select, signal};

pub async fn shutdown_signals() {
    let sigterm = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to register signal handler")
            .recv()
            .await;
    };
    let sigint = async {
        signal::ctrl_c().await.expect("failed to listen for event");
    };

    select! {
        _ = sigterm => {tracing::info!("Received 'terminate' signal, shutting down...");}
        _ = sigint => {tracing::info!("Received 'Ctrl+C' signal, shutting down...");}
    }
}
