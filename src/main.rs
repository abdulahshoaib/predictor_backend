mod handlers;
mod state;

use axum::{
    Router,
    routing::{get, post},
};
use handlers::{
    health::health_check, matches::get_matches, predict::get_predict, predict::put_prediction,
};
use state::AppState;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/matches", get(get_matches))
        .route("/health", get(health_check))
        .route("/predict", get(get_predict))
        .route("/predict", post(put_prediction))
        .with_state(state.clone())
        .layer(cors);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", &state.port))
        .await
        .unwrap();

    println!("Server running...");
    axum::serve(listener, app).await.unwrap();
}
