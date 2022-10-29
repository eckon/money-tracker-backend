use std::collections::HashSet;

use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{self, AccountEntryKind};

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

pub async fn get_account_entry(pool: &PgPool, entry_id: Uuid) -> Result<model::AccountEntry, ()> {
    sqlx::query_as!(
        model::AccountEntry,
        r#"
            SELECT
                id, account_id,  amount, description, tags, event_date,
                kind as "kind: _"
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
            SELECT
                id, account_id, amount, description, tags, event_date,
                kind as "kind: _"
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
    tags: Option<Vec<String>>,
    event_date: chrono::NaiveDate,
) -> Result<model::AccountEntry, ()> {
    let uuid = Uuid::new_v4();

    // sort and remove duplicate values
    let tags = tags
        .unwrap_or(vec![])
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
            INSERT
                INTO account_entry
                    (id, account_id, kind, amount, description, tags, event_date)
                VALUES
                    ($1,         $2,   $3,     $4,          $5,   $6,            $7)
        "#,
        &uuid,
        account_id,
        entry_kind as _,
        amount,
        description,
        &tags[..],
        event_date
    )
    .execute(pool)
    .await
    .map_err(|error| tracing::error!("Error while writing entry for account: {}", error))?;

    get_account_entry(pool, uuid).await
}

pub async fn create_payment(
    pool: &PgPool,
    payer_account_id: Uuid,
    lender_account_id: Uuid,
    amount: i64,
    description: Option<String>,
    event_date: chrono::NaiveDate,
) -> Result<(), ()> {
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

    Ok(())
}

pub async fn get_payment(pool: &PgPool, payment_id: Uuid) -> Result<model::Payment, ()> {
    sqlx::query_as!(
        model::Payment,
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

pub async fn get_payments_by_payer(
    pool: &PgPool,
    payer_account_id: Uuid,
) -> Result<Vec<model::Payment>, ()> {
    sqlx::query_as!(
        model::Payment,
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

pub async fn create_cost(
    pool: &PgPool,
    account_id: Uuid,
    debtor_account_ids: Vec<Uuid>,
    amount: i64,
    description: Option<String>,
    event_date: chrono::NaiveDate,
    tags: Option<Vec<String>>,
) -> Result<(), ()> {
    let cost_uuid = Uuid::new_v4();

    // sort and remove duplicate values
    let tags = tags
        .unwrap_or(vec![])
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    sqlx::query!(
        r#"
            INSERT
                INTO cost
                    (id, account_id, amount, description, event_date, tags)
                VALUES
                    ($1,         $2,     $3,          $4,         $5,   $6)
        "#,
        &cost_uuid,
        account_id,
        amount,
        description,
        event_date,
        &tags[..]
    )
    .execute(pool)
    .await
    .map_err(|error| tracing::error!("Error while writing cost for account: {}", error))?;

    // TODO: delete all data in case one of these fail (maybe its easy to use transaction here?)
    // when cost is created, all linked accounts need a new debt so they know which one needs to repay
    for debt in debtor_account_ids.iter() {
        let debt_uuid = Uuid::new_v4();
        sqlx::query!(
            r#"
                INSERT
                    INTO debt
                        (id, debtor_account_id, cost_id, percentage)
                    VALUES
                        ($1,                $2,      $3,         $4)
            "#,
            &debt_uuid,
            debt,
            &cost_uuid,
            100, // TODO: later on needs to be passed from fe or calcualted somewhere
        )
        .execute(pool)
        .await
        .map_err(|error| tracing::error!("Error while writing cost for account: {}", error))?;
    }

    Ok(())
}

pub async fn get_cost(pool: &PgPool, cost_id: Uuid) -> Result<model::Cost, ()> {
    sqlx::query_as!(
        model::Cost,
        r#"
            SELECT *
            FROM cost
                WHERE id = $1
        "#,
        cost_id
    )
    .fetch_one(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting cost: {}", error))
}

pub async fn get_costs(pool: &PgPool, account_id: Uuid) -> Result<Vec<model::Cost>, ()> {
    sqlx::query_as!(
        model::Cost,
        r#"
            SELECT *
            FROM cost
                WHERE account_id = $1
        "#,
        account_id
    )
    .fetch_all(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting costs of account: {}", error))
}
