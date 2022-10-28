use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{self, AccountEntryKind};

pub async fn get_account(pool: &PgPool, account_id: Uuid) -> Result<model::Account, ()> {
    sqlx::query_as!(
        model::Account,
        r#"
            SELECT * FROM account
            WHERE id = $1
        "#,
        account_id
    )
    .fetch_one(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting account: {}", error))
}

pub async fn create_account(pool: &PgPool, account_name: String) -> Result<model::Account, ()> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        r#"
            INSERT INTO account (id, name)
            VALUES ($1, $2)
        "#,
        &uuid,
        account_name,
    )
    .execute(pool)
    .await
    .map_err(|error| tracing::error!("Error while writing account: {}", error))?;

    get_account(pool, uuid).await
}

pub async fn get_account_entry(pool: &PgPool, entry_id: Uuid) -> Result<model::AccountEntry, ()> {
    sqlx::query_as!(
        model::AccountEntry,
        r#"
            SELECT id, account_id, kind as "kind: _", amount, description
            FROM account_entry
            WHERE id = $1
        "#,
        entry_id
    )
    .fetch_one(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting entry for account: {}", error))
}

pub async fn get_account_entries(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<model::AccountEntry>, ()> {
    sqlx::query_as!(
        model::AccountEntry,
        r#"
            SELECT id, account_id, kind as "kind: _", amount, description
            FROM account_entry
            WHERE account_id = $1
        "#,
        account_id
    )
    .fetch_all(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting entries for account: {}", error))
}

pub async fn create_account_entry(
    pool: &PgPool,
    account_id: Uuid,
    entry_kind: AccountEntryKind,
    amount: i64,
    description: Option<String>,
) -> Result<model::AccountEntry, ()> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        r#"
            INSERT INTO account_entry (id, account_id, kind, amount, description)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        &uuid,
        account_id,
        entry_kind as _,
        amount,
        description
    )
    .execute(pool)
    .await
    .map_err(|error| tracing::error!("Error while writing entry for account: {}", error))?;

    get_account_entry(pool, uuid).await
}
