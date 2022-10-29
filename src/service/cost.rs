use std::collections::{HashMap, HashSet};

use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{entity, dto};
use crate::service;

pub async fn create_cost(
    pool: &PgPool,
    account_id: Uuid,
    debtors: Vec<dto::DebtorDto>,
    amount: i64,
    description: Option<String>,
    event_date: chrono::NaiveDate,
    tags: Option<Vec<String>>,
) -> Result<entity::Cost, ()> {
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
    for debtor in debtors.iter() {
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
            debtor.account_id,
            &cost_uuid,
            debtor.percentage,
        )
        .execute(pool)
        .await
        .map_err(|error| tracing::error!("Error while writing cost for account: {}", error))?;
    }

    get_cost(pool, cost_uuid).await
}

pub async fn get_cost(pool: &PgPool, cost_id: Uuid) -> Result<entity::Cost, ()> {
    sqlx::query_as!(
        entity::Cost,
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

pub async fn get_costs(pool: &PgPool, account_id: Uuid) -> Result<Vec<entity::Cost>, ()> {
    sqlx::query_as!(
        entity::Cost,
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

pub async fn get_all_costs(pool: &PgPool) -> Result<Vec<entity::Cost>, ()> {
    sqlx::query_as!(
        entity::Cost,
        r#"
            SELECT *
            FROM cost
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting costs: {}", error))
}

pub async fn get_account_debt(pool: &PgPool, account_id: Uuid) -> Result<Vec<(Uuid, i64)>, ()> {
    let records = sqlx::query!(
        r#"
            SELECT d.percentage, c.amount, c.account_id
                FROM debt d
                    JOIN cost c ON c.id = d.cost_id
                WHERE d.debtor_account_id = $1
        "#,
        account_id
    )
    .fetch_all(pool)
    .await
    .map_err(|error| tracing::error!("Error while getting debt of account: {}", error))?;

    // calculate the overall debt to the different accounts
    let mut results: HashMap<Uuid, i64> = HashMap::new();
    for record in records.iter() {
        *results.entry(record.account_id).or_insert(0) +=
            // percentage is 0 - 100 so we need to calculate this and divide 100 afterwards
            record.amount * (record.percentage as i64) / 100;
    }

    // transform hashmap into vector
    Ok(results.iter().map(|r| (*r.0, *r.1)).collect::<Vec<_>>())
}

pub async fn get_current_snapshot(pool: &PgPool) -> Result<Vec<dto::CalculatedDebtDto>, ()> {
    let accounts = service::account::get_all_accounts(pool).await?;

    let mut all_debts: Vec<dto::CalculatedDebtDto> = Vec::new();
    for account in accounts.iter() {
        let payments = service::payment::get_account_payments(pool, account.id).await?;
        let debts = get_account_debt(pool, account.id).await?;

        // calculate the overall debt to the different accounts
        let mut results: HashMap<Uuid, i64> = HashMap::new();
        for payment in payments.iter() {
            *results.entry(payment.lender_account_id).or_insert(0) += payment.amount
        }

        for debt in debts.iter() {
            *results.entry(debt.0).or_insert(0) -= debt.1
        }

        // only transform to float at the end to not run into rounding errors
        results.iter().for_each(|result| {
            all_debts.push(dto::CalculatedDebtDto {
                payer_account: account.clone().into(),
                lender_account_id: *result.0,
                amount: (*result.1 as f64) / 100.0,
            })
        })
    }

    Ok(all_debts)
}
