use crate::models::AppState;
use crate::routes::main::main_routes;
use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::Router;
use tower_http::services::ServeDir;

mod main;

pub fn collect_routes() -> Router<AppState> {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    // you can convert handler function to service
    let service = handle_404.into_service();

    let serve_dir = ServeDir::new("assets").not_found_service(service);

    main_routes().fallback_service(serve_dir)
}
