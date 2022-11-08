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

pub fn app() -> Router {
    Router::new()
        .route(
            "/account",
            routing::post(create_account).get(get_all_accounts),
        )
        .route("/account/:account_id", routing::get(get_account))
        .route("/account/:account_id/tags", routing::get(get_account_tags))
}
