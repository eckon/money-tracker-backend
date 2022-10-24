use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{self, AccountEntryKind};

pub async fn get_user(pool: &PgPool, user_id: Uuid) -> Result<model::User, ()> {
    sqlx::query_as!(model::User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await
        .map_err(|error| tracing::error!("Error while getting user: {}", error))
}

pub async fn create_user(pool: &PgPool, user_name: String) -> Result<model::User, ()> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        &uuid,
        user_name,
    )
    .execute(pool)
    .await
    .map_err(|error| tracing::error!("Error while writing user: {}", error))?;

    get_user(pool, uuid).await
}

pub async fn get_account(pool: &PgPool, account_id: Uuid) -> Result<model::Account, ()> {
    sqlx::query_as!(
        model::Account,
        "SELECT * FROM account WHERE id = $1",
        account_id
    )
    .fetch_one(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting account: {}", error))
}

pub async fn create_account(pool: &PgPool, account_name: String) -> Result<model::Account, ()> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO account (id, name) VALUES ($1, $2)",
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
        // TODO: maybe there is another way that uses it more strictly?
        // https://github.com/launchbadge/sqlx/issues/1004
        "SELECT id, account_id, kind as \"kind: _\" FROM account_entry WHERE id = $1",
        entry_id
    )
    .fetch_one(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting entry for account: {}", error))
}

pub async fn create_account_entry(
    pool: &PgPool,
    account_id: Uuid,
    entry_kind: AccountEntryKind,
) -> Result<model::AccountEntry, ()> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO account_entry (id, account_id, kind) VALUES ($1, $2, $3)",
        &uuid,
        account_id,
        // TODO: maybe there is another way that uses it more strictly?
        // https://github.com/launchbadge/sqlx/issues/1004
        entry_kind as _,
    )
    .execute(pool)
    .await
    .map_err(|error| tracing::error!("Error while writing entry for account: {}", error))?;

    get_account_entry(pool, uuid).await
}
