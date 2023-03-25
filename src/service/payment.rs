use sqlx::MySqlPool;
use uuid::Uuid;

use crate::{error::AppError, model::entity};

pub async fn create(
    pool: &MySqlPool,
    payer_account_id: String,
    lender_account_id: String,
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
                    (?,               ?,                ?,     ?,          ?,         ?)
        "#,
        &uuid.to_string(),
        payer_account_id.to_string(),
        lender_account_id.to_string(),
        amount,
        description,
        event_date
    )
    .execute(pool)
    .await?;

    get(pool, uuid.to_string()).await
}

pub async fn delete(pool: &MySqlPool, payment_id: String) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
            DELETE
                FROM payment
                    WHERE id = ?
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

pub async fn get(pool: &MySqlPool, payment_id: String) -> Result<entity::Payment, AppError> {
    Ok(sqlx::query_as!(
        entity::Payment,
        r#"
            SELECT *
            FROM payment
                WHERE id = ?
        "#,
        payment_id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_for_account(
    pool: &MySqlPool,
    payer_account_id: String,
) -> Result<Vec<entity::Payment>, AppError> {
    Ok(sqlx::query_as!(
        entity::Payment,
        r#"
            SELECT *
            FROM payment
                WHERE payer_account_id = ?
        "#,
        payer_account_id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_of_account(
    pool: &MySqlPool,
    payer_account_id: String,
) -> Result<Vec<entity::Payment>, AppError> {
    Ok(sqlx::query_as!(
        entity::Payment,
        r#"
            SELECT *
            FROM payment
                WHERE lender_account_id = ?
        "#,
        payer_account_id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_all(pool: &MySqlPool) -> Result<Vec<entity::Payment>, AppError> {
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
