use axum::{extract::Path, http::StatusCode, routing, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::model;

async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<model::CreateUser>,
) -> Result<Json<model::User>, (StatusCode, String)> {
    let user = db::create_user(&pool, user.name).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "something went wrong".to_string(),
        )
    })?;

    Ok(Json(user))
}

async fn get_user(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<model::User>, (StatusCode, String)> {
    let user = db::get_user(&pool, id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "user not found".to_string()))?;

    Ok(Json(user))
}

pub fn app() -> Router {
    Router::new()
        .route("/user", routing::post(create_user))
        .route("/user/:id", routing::get(get_user))
}
