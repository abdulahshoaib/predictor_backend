mod handlers;
mod state;

use axum::{Router, routing::get};
use handlers::{health::health_check, matches::get_matches};
use state::AppState;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/matches", get(get_matches))
        .route("/health", get(health_check))
        .with_state(state.clone());

    let listener = TcpListener::bind(format!("0.0.0.0:{}", &state.port))
        .await
        .unwrap();

    println!("Server running...");
    axum::serve(listener, app).await.unwrap();
}
