use axum::{extract::Path, http::StatusCode, routing, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::model;

async fn create_account(
    Extension(pool): Extension<PgPool>,
    Json(account): Json<model::CreateAccount>,
) -> Result<Json<model::Account>, (StatusCode, String)> {
    let account = db::create_account(&pool, account.name).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "something went wrong".to_string(),
        )
    })?;

    Ok(Json(account))
}

async fn get_account(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<model::Account>, (StatusCode, String)> {
    let account = db::get_account(&pool, account_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "account not found".to_string()))?;

    Ok(Json(account))
}

async fn create_account_entry(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
    Json(account_entry): Json<model::CreateAccountEntry>,
) -> Result<Json<model::AccountEntry>, (StatusCode, String)> {
    let account = db::get_account(&pool, account_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            "entry of account not found".to_string(),
        )
    })?;

    let entry =
        db::create_account_entry(&pool, account.id, account_entry.kind, account_entry.amount)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "something went wrong".to_string(),
                )
            })?;

    Ok(Json(entry))
}

async fn get_account_entry(
    Extension(pool): Extension<PgPool>,
    Path((_account_id, entry_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<model::AccountEntry>, (StatusCode, String)> {
    let entry = db::get_account_entry(&pool, entry_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            "entry of account not found".to_string(),
        )
    })?;

    Ok(Json(entry))
}

pub fn app() -> Router {
    Router::new()
        .route("/account", routing::post(create_account))
        .route("/account/:account_id", routing::get(get_account))
        .route(
            "/account/:account_id/entry",
            routing::post(create_account_entry),
        )
        .route(
            "/account/:account_id/entry/:entry_id",
            routing::get(get_account_entry),
        )
}
