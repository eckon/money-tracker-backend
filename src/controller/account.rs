use axum::{extract::Path, routing, Extension, Json, Router};
use sqlx::MySqlPool;

use crate::error::AppError;
use crate::model::dto::auth::AuthUser;
use crate::model::dto::{request, response};
use crate::service;

#[utoipa::path(
    post,
    path = "/account",
    request_body = CreateAccountDto,
    responses((status = 200, body = AccountDto)),
    security(("bearer_token" = []))
)]
async fn create_account(
    _user: AuthUser,
    Extension(pool): Extension<MySqlPool>,
    Json(account): Json<request::CreateAccountDto>,
) -> Result<Json<response::AccountDto>, AppError> {
    let account = service::account::create(&pool, account.name).await?;

    Ok(Json(account.into()))
}

#[utoipa::path(
    delete,
    path = "/account/{account_id}",
    params(("account_id" = Uuid, Path,)),
    responses((status = 200), (status = 404)),
    security(("bearer_token" = []))
)]
async fn delete_account(
    _user: AuthUser,
    Extension(pool): Extension<MySqlPool>,
    Path(account_id): Path<String>,
) -> Result<(), AppError> {
    service::account::delete(&pool, account_id).await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/account/{account_id}",
    params(("account_id" = Uuid, Path,)),
    responses((status = 200, body = AccountDto)),
    security(("bearer_token" = []))
)]
async fn get_account(
    _user: AuthUser,
    Extension(pool): Extension<MySqlPool>,
    Path(account_id): Path<String>,
) -> Result<Json<response::AccountDto>, AppError> {
    let account = service::account::get(&pool, account_id).await?;

    Ok(Json(account.into()))
}

#[utoipa::path(
    get,
    path = "/account",
    responses((status = 200, body = [AccountDto])),
    security(("bearer_token" = []))
)]
async fn get_all_accounts(
    _user: AuthUser,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<Vec<response::AccountDto>>, AppError> {
    let accounts = service::account::get_all(&pool).await?;

    let accounts = accounts.iter().cloned().map(Into::into).collect();

    Ok(Json(accounts))
}

#[utoipa::path(
    get,
    path = "/account/{account_id}/tags",
    params(("account_id" = Uuid, Path,)),
    responses((status = 200, body = [String])),
    security(("bearer_token" = []))
)]
async fn get_account_tags(
    _user: AuthUser,
    Extension(pool): Extension<MySqlPool>,
    Path(account_id): Path<String>,
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
        .route(
            "/account/:account_id",
            routing::get(get_account).delete(delete_account),
        )
        .route("/account/:account_id/tags", routing::get(get_account_tags))
}
