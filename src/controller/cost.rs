use axum::{extract::Path, routing, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::dto::auth::AuthUser;
use crate::model::dto::{request, response};
use crate::service;

#[utoipa::path(
    post,
    path = "/account/{account_id}/cost",
    params(("account_id" = Uuid, Path)),
    request_body = CreateCostDto,
    responses((status = 200, body = CostDto)),
    security(("bearer_token" = []))
)]
async fn create_cost(
    _user: AuthUser,
    Extension(pool): Extension<PgPool>,
    Path(account_id): Path<Uuid>,
    Json(cost): Json<request::CreateCostDto>,
) -> Result<Json<response::CostDto>, AppError> {
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
    delete,
    path = "/account/{account_id}/cost/{cost_id}",
    params(request::DeleteCostParams),
    responses((status = 200), (status = 404)),
    security(("bearer_token" = []))
)]
async fn delete_cost(
    _user: AuthUser,
    Extension(pool): Extension<PgPool>,
    Path(params): Path<request::DeleteCostParams>,
) -> Result<(), AppError> {
    service::cost::delete(&pool, params.cost_id).await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/cost",
    responses((status = 200, body = [CostDto])),
    security(("bearer_token" = []))
)]
async fn get_all_costs(
    _user: AuthUser,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<response::CostDto>>, AppError> {
    let costs = service::cost::get_all(&pool).await?;

    let costs = costs.iter().cloned().map(Into::into).collect();

    Ok(Json(costs))
}

#[utoipa::path(
    get,
    path = "/snapshot",
    responses((status = 200, body = [CalculatedDebtDto])),
    security(("bearer_token" = []))
)]
async fn get_current_snapshot(
    _user: AuthUser,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<response::CalculatedDebtDto>>, AppError> {
    let debt = service::cost::get_current_snapshot(&pool).await?;

    Ok(Json(debt))
}

pub fn app() -> Router {
    Router::new()
        .route("/account/:account_id/cost", routing::post(create_cost))
        .route(
            "/account/:account_id/cost/:cost_id",
            routing::delete(delete_cost),
        )
        .route("/cost", routing::get(get_all_costs))
        .route("/snapshot", routing::get(get_current_snapshot))
}
