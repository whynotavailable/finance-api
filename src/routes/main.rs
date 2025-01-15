use crate::errors::{json_ok, AppError, AppResult, JsonResult};
use crate::models::{AppState, SimpleResponse};
use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use uuid::Uuid;

#[derive(Serialize)]
struct Status {
    status: String,
}

async fn status() -> JsonResult<Status> {
    json_ok(Status {
        status: "Ok".to_string(),
    })
}

#[derive(Deserialize)]
struct AddEntryBody {
    account_id: Uuid,
}

async fn add_entry(
    State(state): State<AppState>,
    Json(body): Json<AddEntryBody>,
) -> JsonResult<SimpleResponse> {
    sqlx::query("CALL new_entry($1, $2)")
        .bind(Uuid::new_v4())
        .bind(body.account_id)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::server_error(e.to_string()))?;

    json_ok(SimpleResponse::new("Created!".to_string()))
}

#[derive(Deserialize)]
struct GetBalanceQuery {
    account_id: Uuid,
}

#[derive(Serialize)]
struct GetBalanceResponse {
    amount: BigDecimal,
}

async fn get_balance(
    State(state): State<AppState>,
    query: Query<GetBalanceQuery>,
) -> AppResult<Json<GetBalanceResponse>> {
    let results: (BigDecimal,) = sqlx::query_as("SELECT get_balance($1)")
        .bind(query.account_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| AppError::server_error(e.to_string()))?;

    Ok(Json(GetBalanceResponse {
        amount: results.0.with_scale(2),
    }))
}

pub fn main_routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(status))
        .route("/balance", get(get_balance))
        .route("/entry", post(add_entry))
        .layer(CorsLayer::permissive())
}
