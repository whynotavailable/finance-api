use crate::errors::AppResult;
use crate::models::{AppState, Status};
use axum::routing::get;
use axum::{Json, Router};
use tower_http::cors::CorsLayer;

pub async fn status() -> AppResult<Json<Status>> {
    Ok(Json(Status {
        status: "Ok".to_string(),
    }))
}

pub fn main_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(status))
        .layer(CorsLayer::permissive())
}
