use serde_json::json;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum AppError {
    Service(String),
    InternalServer(String),
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::Service(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
            Self::InternalServer(msg) => {
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
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::InternalServer(value.to_string()),
        }
    }
}
