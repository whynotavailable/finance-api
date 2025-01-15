use crate::errors::{AppError, AppResult};
use crate::models::{AppState, SimpleResponse, Status};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

async fn status() -> AppResult<Json<Status>> {
    Ok(Json(Status {
        status: "Ok".to_string(),
    }))
}

#[derive(Deserialize)]
struct AddEntryBody {
    account_id: Uuid,
}

async fn add_entry(
    State(state): State<AppState>,
    Json(body): Json<AddEntryBody>,
) -> AppResult<Json<SimpleResponse>> {
    sqlx::query("CALL new_entry($1, $2)")
        .bind(Uuid::new_v4())
        .bind(body.account_id)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(SimpleResponse::new("Created!".to_string())))
}

pub fn main_routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(status))
        .route("/entry", post(add_entry))
        .layer(CorsLayer::permissive())
}
