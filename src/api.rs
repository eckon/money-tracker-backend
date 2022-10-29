use std::collections::HashSet;

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

    let entries = entries
        .iter()
        .cloned()
        .map(|e| e.into())
        .collect::<Vec<model::AccountEntryDto>>();

    let result = model::AccountDto {
        entries: if entries.len() <= 0 {
            None
        } else {
            Some(entries)
        },
        ..account.into()
    };

    Ok(Json(result))
}

async fn get_all_accounts(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<model::AccountDto>>, (StatusCode, String)> {
    let accounts = db::get_all_accounts(&pool)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "accounts do not exist".to_string()))?;

    let accounts = accounts.iter().cloned().map(|a| a.into()).collect();

    Ok(Json(accounts))
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
    let entry = db::create_account_entry(
        &pool,
        account.id,
        account_entry.kind,
        amount,
        account_entry.description,
        account_entry.tags,
        account_entry.event_date,
    )
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

// TODO: update with new model
async fn get_account_tags(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let entries = db::get_account_entries(&pool, account_id)
        .await
        .unwrap_or(vec![]);

    // map tags, sort and remove duplicate values
    let result = entries
        .iter()
        .cloned()
        .filter_map(|e| e.tags)
        .flatten()
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    Ok(Json(result))
}

async fn create_cost(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
    Json(cost): Json<model::CreateCostDto>,
) -> Result<(), (StatusCode, String)> {
    let amount = (cost.amount * 100.0) as i64;
    db::create_cost(
        &pool,
        account_id,
        cost.debtor_account_ids,
        amount,
        cost.description,
        cost.event_date,
        cost.tags,
    )
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "something went wrong".to_string(),
        )
    })?;
    Ok(())
}

async fn create_payment(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
    Json(payment): Json<model::CreatePaymentDto>,
) -> Result<(), (StatusCode, String)> {
    let amount = (payment.amount * 100.0) as i64;
    db::create_payment(
        &pool,
        account_id,
        payment.lender_account_id,
        amount,
        payment.description,
        payment.event_date,
    )
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "something went wrong".to_string(),
        )
    })?;
    Ok(())
}

pub fn app() -> Router {
    Router::new()
        .route("/account", routing::post(create_account))
        .route("/account", routing::get(get_all_accounts))
        .route("/account/:account_id", routing::get(get_account))
        .route(
            "/account/:account_id/entry",
            routing::post(create_account_entry),
        )
        .route(
            "/account/:account_id/entry/:entry_id",
            routing::get(get_account_entry),
        )
        .route("/account/:account_id/tags", routing::get(get_account_tags))
        // .route("/account/:account_id/cost/:cost_id", routing::get(get_cost))
        .route("/account/:account_id/cost", routing::post(create_cost))
        // .route("/account/:account_id/payment/:payment_id", routing::get(get_payment))
        .route(
            "/account/:account_id/payment",
            routing::post(create_payment),
        )
}
