use axum::{extract::Path, http::StatusCode, routing, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::dto;
use crate::service;

async fn create_account(
    Extension(pool): Extension<PgPool>,
    Json(account): Json<dto::CreateAccountDto>,
) -> Result<Json<dto::AccountDto>, (StatusCode, String)> {
    let account = service::account::create_account(&pool, account.name)
        .await
        .map_err(|_| {
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
) -> Result<Json<dto::AccountDto>, (StatusCode, String)> {
    let account = service::account::get_account(&pool, account_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "account not found".to_string()))?;

    Ok(Json(account.into()))
}

async fn get_all_accounts(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<dto::AccountDto>>, (StatusCode, String)> {
    let accounts = service::account::get_all_accounts(&pool)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "accounts do not exist".to_string()))?;

    let accounts = accounts.iter().cloned().map(|a| a.into()).collect();

    Ok(Json(accounts))
}

async fn get_account_tags(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let tags = service::account::get_tags(&pool, account_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "tags do not exist".to_string()))?;

    Ok(Json(tags))
}

async fn create_cost(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
    Json(cost): Json<dto::CreateCostDto>,
) -> Result<Json<dto::CostDto>, (StatusCode, String)> {
    let amount = (cost.amount * 100.0) as i64;
    let cost = service::cost::create_cost(
        &pool,
        account_id,
        cost.debtors,
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

    Ok(Json(cost.into()))
}

async fn get_all_costs(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<dto::CostDto>>, (StatusCode, String)> {
    let costs = service::cost::get_all_costs(&pool)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "costs do not exist".to_string()))?;

    let costs = costs.iter().cloned().map(|a| a.into()).collect();

    Ok(Json(costs))
}

async fn create_payment(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
    Json(payment): Json<dto::CreatePaymentDto>,
) -> Result<Json<dto::PaymentDto>, (StatusCode, String)> {
    let amount = (payment.amount * 100.0) as i64;
    let payment = service::payment::create_payment(
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

    Ok(Json(payment.into()))
}

async fn get_all_payment(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<dto::PaymentDto>>, (StatusCode, String)> {
    let payments = service::payment::get_all_payment(&pool)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "payments do not exist".to_string()))?;

    let payments = payments.iter().cloned().map(|a| a.into()).collect();

    Ok(Json(payments))
}

async fn get_current_snapshot(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<dto::CalculatedDebtDto>>, (StatusCode, String)> {
    let debt = service::cost::get_current_snapshot(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "something went wrong".to_string(),
            )
        })?;

    Ok(Json(debt))
}

pub fn app() -> Router {
    Router::new()
        .route(
            "/account",
            routing::post(create_account).get(get_all_accounts),
        )
        .route("/account/:account_id", routing::get(get_account))
        .route("/account/:account_id/tags", routing::get(get_account_tags))
        .route("/account/:account_id/cost", routing::post(create_cost))
        .route(
            "/account/:account_id/payment",
            routing::post(create_payment),
        )
        .route("/cost", routing::get(get_all_costs))
        .route("/payment", routing::get(get_all_payment))
        .route("/snapshot", routing::get(get_current_snapshot))
}
