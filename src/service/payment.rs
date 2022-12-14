use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, model::entity};

pub async fn create(
    pool: &PgPool,
    payer_account_id: Uuid,
    lender_account_id: Uuid,
    amount: i64,
    description: Option<String>,
    event_date: chrono::NaiveDate,
) -> Result<entity::Payment, AppError> {
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
    .await?;

    get(pool, uuid).await
}

pub async fn delete(pool: &PgPool, payment_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
            DELETE
                FROM payment
                    WHERE id = $1
        "#,
        payment_id,
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

pub async fn get(pool: &PgPool, payment_id: Uuid) -> Result<entity::Payment, AppError> {
    Ok(sqlx::query_as!(
        entity::Payment,
        r#"
            SELECT *
            FROM payment
                WHERE id = $1
        "#,
        payment_id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_for_account(
    pool: &PgPool,
    payer_account_id: Uuid,
) -> Result<Vec<entity::Payment>, AppError> {
    Ok(sqlx::query_as!(
        entity::Payment,
        r#"
            SELECT *
            FROM payment
                WHERE payer_account_id = $1
        "#,
        payer_account_id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_of_account(
    pool: &PgPool,
    payer_account_id: Uuid,
) -> Result<Vec<entity::Payment>, AppError> {
    Ok(sqlx::query_as!(
        entity::Payment,
        r#"
            SELECT *
            FROM payment
                WHERE lender_account_id = $1
        "#,
        payer_account_id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<entity::Payment>, AppError> {
    Ok(sqlx::query_as!(
        entity::Payment,
        r#"
            SELECT *
            FROM payment
        "#,
    )
    .fetch_all(pool)
    .await?)
}
