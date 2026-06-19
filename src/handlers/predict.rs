use crate::state::AppState;
use axum::{Json, extract::State, http::HeaderMap};
use serde::Deserialize;
use serde_json::{Value, json};

pub async fn get_predict(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, String> {
    log::info!("get /predict called");

    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| "Missing or invalid Authorization header".to_string())?;

    let res = state
        .client
        .get(format!(
            "{}/rest/v1/predictions?select=*",
            state.supabase_url
        ))
        .header("apikey", &state.service_key)
        .header("Authorization", auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data = res
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    log::info!("Fetched predictions: {:?}", data);

    Ok(Json(data))
}

#[derive(Deserialize)]
pub struct PredictionPayload {
    pub match_id: i64,
    pub user_id: String,
    pub prediction_choice: String,
}

pub async fn put_prediction(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<PredictionPayload>,
) -> Result<Json<Value>, String> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| "Missing or invalid Authorization header".to_string())?;

    let new_prediction = json!({
        "match_id": payload.match_id,
        "user_id": payload.user_id,
        "prediction_choice": payload.prediction_choice,
    });

    let res = state
        .client
        .post(format!(
            "{}/rest/v1/predictions?on_conflict=match_id,user_id",
            state.supabase_url
        ))
        .header("apikey", &state.service_key)
        .header("Authorization", auth_header)
        .header(
            "Prefer",
            "return=representation,resolution=merge-duplicates",
        )
        .json(&new_prediction)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: Value = res.json().await.map_err(|e| e.to_string())?;

    Ok(Json(data))
}
