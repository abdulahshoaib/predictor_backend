mod handlers;
mod state;

use axum::{Router, routing::get};
use handlers::matches::get_matches;
use state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    println!("Supabase URL: {}", state.supabase_url);
    println!("Supabase Service Key: {}", state.service_key);

    let app = Router::new()
        .route("/matches", get(get_matches))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
