use sqlx::PgPool;
use uuid::Uuid;

use crate::model;

pub async fn get_account(pool: &PgPool, account_id: Uuid) -> Result<model::Account, ()> {
    sqlx::query_as!(
        model::Account,
        r#"
            SELECT *
            FROM account
                WHERE id = $1
        "#,
        account_id
    )
    .fetch_one(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting account: {}", error))
}

pub async fn get_all_accounts(pool: &PgPool) -> Result<Vec<model::Account>, ()> {
    sqlx::query_as!(
        model::Account,
        r#"
            SELECT * FROM account
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting accounts: {}", error))
}

pub async fn create_account(pool: &PgPool, account_name: String) -> Result<model::Account, ()> {
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
    .await
    .map_err(|error| tracing::error!("Error while writing account: {}", error))?;

    get_account(pool, uuid).await
}