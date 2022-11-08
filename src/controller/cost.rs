use axum::{extract::Path, routing, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::dto;
use crate::service;

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
        .route("/account/:account_id/cost", routing::post(create_cost))
        .route("/cost", routing::get(get_all_costs))
        .route("/snapshot", routing::get(get_current_snapshot))
}
