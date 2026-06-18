use crate::state::AppState;
pub(crate) use axum::{Json, extract::State};
use serde_json::{Value, json};

pub async fn get_matches(State(state): State<AppState>) -> Result<Json<Value>, String> {
    log::info!("GET /matches called");
    let res = state
        .client
        .get(format!("{}/rest/v1/matches?select=*", state.supabase_url))
        .header("apikey", &state.service_key)
        .header("Authorization", format!("Bearer {}", &state.service_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: Vec<Value> = res.json().await.map_err(|e| e.to_string())?;

    let (full_time, upcoming): (Vec<Value>, Vec<Value>) = data
        .into_iter()
        .partition(|m| m.get("status").and_then(|s| s.as_str()) == Some("completed"));

    let grouped_data = json!({
        "upcoming": upcoming,
        "full_time": full_time
    });

    log::info!("Fetched matches: {:?}", grouped_data);

    Ok(Json(grouped_data))
}
