use axum::{extract::Path, routing, Extension, Json, Router};
use sqlx::MySqlPool;

use crate::error::AppError;
use crate::helper::Conversion;
use crate::model::dto::auth::AuthUser;
use crate::model::dto::{request, response};
use crate::service;

#[utoipa::path(
    post,
    path = "/account/{account_id}/payment",
    params(("account_id" = Uuid, Path,)),
    request_body = CreatePaymentDto,
    responses((status = 200, body = PaymentDto)),
    security(("bearer_token" = []))
)]
async fn create_payment(
    _user: AuthUser,
    Extension(pool): Extension<MySqlPool>,
    Path(account_id): Path<String>,
    Json(payment): Json<request::CreatePaymentDto>,
) -> Result<Json<response::PaymentDto>, AppError> {
    let amount = Conversion::to_int(payment.amount);
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
    params(request::DeletePaymentParams),
    responses((status = 200), (status = 404)),
    security(("bearer_token" = []))
)]
async fn delete_payment(
    _user: AuthUser,
    Extension(pool): Extension<MySqlPool>,
    Path(params): Path<request::DeletePaymentParams>,
) -> Result<(), AppError> {
    service::payment::delete(&pool, params.payment_id).await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/payment",
    responses((status = 200, body = [PaymentDto])),
    security(("bearer_token" = []))
)]
async fn get_all_payment(
    _user: AuthUser,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<Vec<response::PaymentDto>>, AppError> {
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
