use axum::{extract::Path, routing, Extension, Json, Router};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::dto;
use crate::service;

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
    delete,
    path = "/account/{account_id}/payment/{payment_id}",
    params(dto::DeletePaymentParams),
    responses((status = 200), (status = 404)),
)]
async fn delete_payment(
    Extension(pool): Extension<PgPool>,
    Path(params): Path<dto::DeletePaymentParams>,
) -> Result<(), AppError> {
    service::payment::delete(&pool, params.payment_id).await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/payment",
    responses((status = 200, body = [PaymentDto])),
)]
async fn get_all_payment(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<dto::PaymentDto>>, AppError> {
    let payments = service::payment::get_all(&pool).await?;

    let payments = payments.iter().cloned().map(Into::into).collect();

    Ok(Json(payments))
}

pub fn app() -> Router {
    Router::new()
        .route(
            "/account/:account_id/payment",
            routing::post(create_payment),
        )
        .route(
            "/account/:account_id/payment/:payment_id",
            routing::delete(delete_payment),
        )
        .route("/payment", routing::get(get_all_payment))
}
