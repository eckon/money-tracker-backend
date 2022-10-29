use sqlx::PgPool;
use uuid::Uuid;

use crate::model::db;

pub async fn create_payment(
    pool: &PgPool,
    payer_account_id: Uuid,
    lender_account_id: Uuid,
    amount: i64,
    description: Option<String>,
    event_date: chrono::NaiveDate,
) -> Result<db::Payment, ()> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        r#"
            INSERT
                INTO payment
                    (id, payer_account_id, lender_account_id, amount, description, event_date)
                VALUES
                    ($1,               $2,                $3,     $4,          $5,         $6)
        "#,
        &uuid,
        payer_account_id,
        lender_account_id,
        amount,
        description,
        event_date
    )
    .execute(pool)
    .await
    .map_err(|error| tracing::error!("Error while writing payment for account: {}", error))?;

    get_payment(pool, uuid).await
}

pub async fn get_payment(pool: &PgPool, payment_id: Uuid) -> Result<db::Payment, ()> {
    sqlx::query_as!(
        db::Payment,
        r#"
            SELECT *
            FROM payment
                WHERE id = $1
        "#,
        payment_id
    )
    .fetch_one(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting payment: {}", error))
}

pub async fn get_account_payments(
    pool: &PgPool,
    payer_account_id: Uuid,
) -> Result<Vec<db::Payment>, ()> {
    sqlx::query_as!(
        db::Payment,
        r#"
            SELECT *
            FROM payment
                WHERE payer_account_id = $1
        "#,
        payer_account_id
    )
    .fetch_all(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting payments of account: {}", error))
}

pub async fn get_all_payment(pool: &PgPool) -> Result<Vec<db::Payment>, ()> {
    sqlx::query_as!(
        db::Payment,
        r#"
            SELECT *
            FROM payment
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting payments: {}", error))
}
