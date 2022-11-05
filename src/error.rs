use serde_json::json;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(Debug)]
pub enum AppError {
    ServiceError(String),
    InternalServerError(String),
    NotFoundError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ServiceError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFoundError => (StatusCode::NOT_FOUND, "not found".into()),
            AppError::InternalServerError(msg) => {
                tracing::error!("Error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".into(),
                )
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => AppError::NotFoundError,
            _ => AppError::InternalServerError(value.to_string()),
        }
    }
}
