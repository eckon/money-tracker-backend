use std::collections::HashSet;

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::entity;
use crate::service;

pub async fn get(pool: &PgPool, account_id: Uuid) -> Result<entity::Account, AppError> {
    Ok(sqlx::query_as!(
        entity::Account,
        r#"
            SELECT *
            FROM account
                WHERE id = $1
        "#,
        account_id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<entity::Account>, AppError> {
    Ok(sqlx::query_as!(
        entity::Account,
        r#"
            SELECT * FROM account
        "#
    )
    .fetch_all(pool)
    .await?)
}

pub async fn create(pool: &PgPool, account_name: String) -> Result<entity::Account, AppError> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        r#"
            INSERT
                INTO account
                    (id, name)
                VALUES
                    ($1,   $2)
        "#,
        &uuid,
        account_name,
    )
    .execute(pool)
    .await?;

    get(pool, uuid).await
}

pub async fn delete(pool: &PgPool, account_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
            DELETE
                FROM account
                    WHERE id = $1
        "#,
        account_id,
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

pub async fn get_tags(pool: &PgPool, account_id: Uuid) -> Result<Vec<String>, AppError> {
    let costs = service::cost::get_for_account(pool, account_id).await?;

    // map tags, sort and remove duplicate values
    let result = costs
        .iter()
        .cloned()
        .filter_map(|e| e.tags)
        .flatten()
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    if result.is_empty() {
        return Err(AppError::NotFound);
    }

    Ok(result)
}
