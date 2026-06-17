pub(crate) use crate::state::AppState;
use axum::{Json, extract::State};

pub async fn get_matches(State(state): State<AppState>) -> Result<Json<serde_json::Value>, String> {
    log::info!("GET /matches called");
    let res = state
        .client
        .get(format!("{}/rest/v1/matches?select=*", state.supabase_url))
        .header("apikey", &state.service_key)
        .header("Authorization", format!("Bearer {}", &state.service_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data = res
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    log::info!("Fetched matches: {:?}", data);

    Ok(Json(data))
}
