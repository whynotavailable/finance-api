use crate::models::AppState;
use crate::routes::main::main_routes;
use axum::Router;

mod main;

pub fn collect_routes() -> Router<AppState> {
    main_routes()
}
