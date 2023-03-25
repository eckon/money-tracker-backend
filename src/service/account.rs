use std::collections::HashSet;

use sqlx::MySqlPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::dto::response::CostDto;
use crate::model::entity;
use crate::service;

pub async fn get(pool: &MySqlPool, account_id: String) -> Result<entity::Account, AppError> {
    Ok(sqlx::query_as!(
        entity::Account,
        r#"
            SELECT *
            FROM account
                WHERE id = ?
        "#,
        account_id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_all(pool: &MySqlPool) -> Result<Vec<entity::Account>, AppError> {
    Ok(sqlx::query_as!(
        entity::Account,
        r#"
            SELECT * FROM account
        "#
    )
    .fetch_all(pool)
    .await?)
}

pub async fn create(pool: &MySqlPool, account_name: String) -> Result<entity::Account, AppError> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        r#"
            INSERT
                INTO account
                    (id, name)
                VALUES
                    (?,   ?)
        "#,
        &uuid.to_string(),
        account_name,
    )
    .execute(pool)
    .await?;

    get(pool, uuid.to_string()).await
}

pub async fn delete(pool: &MySqlPool, account_id: String) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
            DELETE
                FROM account
                    WHERE id = ?
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

pub async fn get_tags(pool: &MySqlPool, account_id: String) -> Result<Vec<String>, AppError> {
    let costs = service::cost::get_for_account(pool, account_id).await?;
    let cost_dtos: Vec<CostDto> = costs.iter().map(|e| e.clone().into()).collect();

    // map tags, sort and remove duplicate values
    let result = cost_dtos
        .iter()
        .cloned()
        .filter_map(|e| e.tags)
        .flatten()
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    Ok(result)
}
