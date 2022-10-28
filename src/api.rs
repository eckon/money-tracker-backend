use axum::{extract::Path, http::StatusCode, routing, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
use crate::model;

async fn create_account(
    Extension(pool): Extension<PgPool>,
    Json(account): Json<model::CreateAccountDto>,
) -> Result<Json<model::AccountDto>, (StatusCode, String)> {
    let account = db::create_account(&pool, account.name).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "something went wrong".to_string(),
        )
    })?;

    Ok(Json(account.into()))
}

async fn get_account(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<model::AccountDto>, (StatusCode, String)> {
    let account = db::get_account(&pool, account_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "account not found".to_string()))?;

    let entries = db::get_account_entries(&pool, account_id)
        .await
        .unwrap_or(vec![]);

    let result = model::AccountDto {
        entry: entries
            .iter()
            .cloned()
            .map(|e| e.into())
            .collect(),
        ..account.into()
    };

    Ok(Json(result))
}

async fn create_account_entry(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
    Json(account_entry): Json<model::CreateAccountEntryDto>,
) -> Result<Json<model::AccountEntryDto>, (StatusCode, String)> {
    let account = db::get_account(&pool, account_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            "entry of account not found".to_string(),
        )
    })?;

    // tansform api amount to db amount (stored as int not as float)
    let amount = (account_entry.amount * 100.0) as i64;
    let entry = db::create_account_entry(&pool, account.id, account_entry.kind, amount)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "something went wrong".to_string(),
            )
        })?;

    Ok(Json(entry.into()))
}

async fn get_account_entry(
    Extension(pool): Extension<PgPool>,
    Path((_account_id, entry_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<model::AccountEntryDto>, (StatusCode, String)> {
    let entry = db::get_account_entry(&pool, entry_id).await.map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            "entry of account not found".to_string(),
        )
    })?;

    Ok(Json(entry.into()))
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
