use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing, Extension, Json, Router,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db;

async fn create_user(
    Extension(pool): Extension<PgPool>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let user = db::create_user(&pool, name).await.unwrap();

    (StatusCode::OK, Json(user))
}

async fn get_user(Extension(pool): Extension<PgPool>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let user = db::get_user(&pool, id).await.unwrap();

    (StatusCode::OK, Json(user))
}

pub fn get_router() -> Router {
    Router::new()
        .route("/user/create/:name", routing::get(create_user))
        .route("/user/:id", routing::get(get_user))
}
