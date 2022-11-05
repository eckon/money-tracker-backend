use axum::{extract::Path, routing, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::dto;
use crate::service;

#[utoipa::path(
    post,
    path = "/account",
    request_body = CreateAccountDto,
    responses((status = 200, body = AccountDto)),
)]
async fn create_account(
    Extension(pool): Extension<PgPool>,
    Json(account): Json<dto::CreateAccountDto>,
) -> Result<Json<dto::AccountDto>, AppError> {
    let account = service::account::create(&pool, account.name).await?;

    Ok(Json(account.into()))
}

#[utoipa::path(
    get,
    path = "/account/{account_id}",
    params(("account_id" = Uuid, Path)),
    responses((status = 200, body = AccountDto)),
)]
async fn get_account(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<dto::AccountDto>, AppError> {
    let account = service::account::get(&pool, account_id).await?;

    Ok(Json(account.into()))
}

#[utoipa::path(
    get,
    path = "/account",
    responses((status = 200, body = [AccountDto])),
)]
async fn get_all_accounts(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<dto::AccountDto>>, AppError> {
    let accounts = service::account::get_all(&pool).await?;

    let accounts = accounts.iter().cloned().map(Into::into).collect();

    Ok(Json(accounts))
}

#[utoipa::path(
    get,
    path = "/account/{account_id}/tags",
    params(("account_id" = Uuid, Path)),
    responses((status = 200, body = [String])),
)]
async fn get_account_tags(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<Vec<String>>, AppError> {
    let tags = service::account::get_tags(&pool, account_id).await?;

    Ok(Json(tags))
}

#[utoipa::path(
    post,
    path = "/account/{account_id}/cost",
    params(("account_id" = Uuid, Path)),
    request_body = CreateCostDto,
    responses((status = 200, body = CostDto)),
)]
async fn create_cost(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
    Json(cost): Json<dto::CreateCostDto>,
) -> Result<Json<dto::CostDto>, AppError> {
    #[allow(clippy::cast_possible_truncation)]
    let amount = (cost.amount * 100.0) as i64;
    let cost = service::cost::create(
        &pool,
        account_id,
        cost.debtors,
        amount,
        cost.description,
        cost.event_date,
        cost.tags,
    )
    .await?;

    Ok(Json(cost.into()))
}

#[utoipa::path(
    get,
    path = "/cost",
    responses((status = 200, body = [CostDto])),
)]
async fn get_all_costs(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<dto::CostDto>>, AppError> {
    let costs = service::cost::get_all(&pool).await?;

    let costs = costs.iter().cloned().map(Into::into).collect();

    Ok(Json(costs))
}

#[utoipa::path(
    post,
    path = "/account/{account_id}/payment",
    params(("account_id" = Uuid, Path)),
    request_body = CreatePaymentDto,
    responses((status = 200, body = PaymentDto)),
)]
async fn create_payment(
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
    Json(payment): Json<dto::CreatePaymentDto>,
) -> Result<Json<dto::PaymentDto>, AppError> {
    #[allow(clippy::cast_possible_truncation)]
    let amount = (payment.amount * 100.0) as i64;
    let payment = service::payment::create(
        &pool,
        account_id,
        payment.lender_account_id,
        amount,
        payment.description,
        payment.event_date,
    )
    .await?;

    Ok(Json(payment.into()))
}

#[utoipa::path(
    get,
    path = "/account",
    responses((status = 200, body = [PaymentDto])),
)]
async fn get_all_payment(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<dto::PaymentDto>>, AppError> {
    let payments = service::payment::get_all(&pool).await?;

    let payments = payments.iter().cloned().map(Into::into).collect();

    Ok(Json(payments))
}

#[utoipa::path(
    get,
    path = "/snapshot",
    responses((status = 200, body = [CalculatedDebtDto])),
)]
async fn get_current_snapshot(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<dto::CalculatedDebtDto>>, AppError> {
    let debt = service::cost::get_current_snapshot(&pool).await?;

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
