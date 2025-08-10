use axum::{
    routing::get,
    Router,
    Json,
    extract::State,
};
use serde::Serialize;
use tokio::signal;
use tokio::sync::oneshot;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
}

async fn status_handler() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "running".to_string(),
    })
}

async fn stop_handler(State(state): State<AppState>) -> &'static str {
    if let Some(tx) = state.shutdown_tx.lock().unwrap().take() {
        let _ = tx.send(());
        "Shutting down..."
    } else {
        "Shutdown signal already sent or not available."
    }
}

#[tokio::main]
async fn main() {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let app_state = AppState {
        shutdown_tx: Arc::new(Mutex::new(Some(shutdown_tx))),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, Quiz Server!" }))
        .route("/status", get(status_handler))
        .route("/stop", get(stop_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Quiz server listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            signal::ctrl_c().await.unwrap();
            println!("Ctrl-C received, shutting down gracefully.");
            // Also listen for the oneshot signal from the /stop endpoint
            let _ = shutdown_rx.await;
            println!("Shutdown signal received, shutting down.");
        })
        .await
        .unwrap();
}