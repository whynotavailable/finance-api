use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

/// Global error type
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

// This is to fix the warning
#[allow(dead_code)]
impl AppError {
    pub fn status(code: StatusCode) -> AppError {
        AppError {
            code,
            message: "".to_string(),
        }
    }

    pub fn new(code: StatusCode, message: String) -> AppError {
        AppError { code, message }
    }

    pub fn not_found() -> AppError {
        AppError {
            code: StatusCode::NOT_FOUND,
            message: "Not Found".to_string(),
        }
    }

    pub fn server_error(message: String) -> AppError {
        AppError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }

    /// Shorthand for server_error
    pub fn se(message: String) -> AppError {
        AppError::server_error(message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

/// Use this for most functions that return a result
pub type AppResult<T> = Result<T, AppError>;
pub type JsonResult<T> = AppResult<Json<T>>;

pub fn json_ok<T>(o: T) -> JsonResult<T> {
    Ok(Json(o))
}
