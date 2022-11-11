use std::collections::{HashMap, HashSet};

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::{dto, entity};
use crate::service;

pub async fn create(
    pool: &PgPool,
    account_id: Uuid,
    debtors: Vec<dto::DebtorDto>,
    amount: i64,
    description: Option<String>,
    event_date: chrono::NaiveDate,
    tags: Option<Vec<String>>,
) -> Result<entity::Cost, AppError> {
    let percentage_sum = debtors.iter().map(|p| p.percentage).sum::<i16>();
    if percentage_sum != 100 {
        return Err(AppError::Service(format!(
            "sum of all debtors needs to be 100% - currently is {percentage_sum}%"
        )));
    }

    let cost_uuid = Uuid::new_v4();

    // sort and remove duplicate values
    let tags = tags
        .unwrap_or_default()
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
    .await?;

    for debtor in &debtors {
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
        .await?;
    }

    get(pool, cost_uuid).await
}

pub async fn delete(pool: &PgPool, cost_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
            DELETE
                FROM cost
                    WHERE id = $1
        "#,
        cost_id,
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

pub async fn get(pool: &PgPool, cost_id: Uuid) -> Result<entity::Cost, AppError> {
    Ok(sqlx::query_as!(
        entity::Cost,
        r#"
            SELECT *
            FROM cost
                WHERE id = $1
        "#,
        cost_id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_for_account(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<entity::Cost>, AppError> {
    Ok(sqlx::query_as!(
        entity::Cost,
        r#"
            SELECT *
            FROM cost
                WHERE account_id = $1
        "#,
        account_id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<entity::Cost>, AppError> {
    Ok(sqlx::query_as!(
        entity::Cost,
        r#"
            SELECT *
            FROM cost
        "#,
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_debts_of_account(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<(Uuid, i64)>, AppError> {
    let records = sqlx::query!(
        r#"
            SELECT d.percentage, d.debtor_account_id, c.amount, c.account_id
                FROM debt d
                    JOIN cost c ON c.id = d.cost_id
                WHERE c.account_id = $1
        "#,
        account_id
    )
    .fetch_all(pool)
    .await?;

    // calculate the overall debt to the different accounts
    let mut results: HashMap<Uuid, i64> = HashMap::new();
    for record in &records {
        *results.entry(record.debtor_account_id).or_insert(0) +=
            // percentage is 0 - 100 so we need to calculate this and divide 100 afterwards
            record.amount * i64::from(record.percentage) / 100;
    }

    // transform hashmap into vector
    Ok(results.iter().map(|r| (*r.0, *r.1)).collect::<Vec<_>>())
}

pub async fn get_debts_for_account(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<(Uuid, i64)>, AppError> {
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
    .await?;

    // calculate the overall debt to the different accounts
    let mut results: HashMap<Uuid, i64> = HashMap::new();
    for record in &records {
        *results.entry(record.account_id).or_insert(0) +=
            // percentage is 0 - 100 so we need to calculate this and divide 100 afterwards
            record.amount * i64::from(record.percentage) / 100;
    }

    // transform hashmap into vector
    Ok(results.iter().map(|r| (*r.0, *r.1)).collect::<Vec<_>>())
}

pub async fn get_current_snapshot(pool: &PgPool) -> Result<Vec<dto::CalculatedDebtDto>, AppError> {
    let accounts = service::account::get_all(pool).await?;

    let mut all_debts: Vec<dto::CalculatedDebtDto> = Vec::new();
    for account in &accounts {
        let payed_payments = service::payment::get_for_account(pool, account.id).await?;
        let given_payments = service::payment::get_of_account(pool, account.id).await?;
        let to_pay_debts = get_debts_for_account(pool, account.id).await?;
        let being_payed_debts = get_debts_of_account(pool, account.id).await?;

        // calculate the overall debt from payer account to lender account
        let mut results: HashMap<Uuid, i64> = HashMap::new();

        // payer account pays to lender account via payment
        for payment in &payed_payments {
            *results.entry(payment.lender_account_id).or_insert(0) += payment.amount;
        }

        // lender account could have payed to payer account via payment
        for payment in &given_payments {
            *results.entry(payment.payer_account_id).or_insert(0) -= payment.amount;
        }

        // lender account could have debts to payer account via debts
        for debt in &being_payed_debts {
            *results.entry(debt.0).or_insert(0) += debt.1;
        }

        // payer account could have debts to lender account via debts
        for debt in to_pay_debts {
            *results.entry(debt.0).or_insert(0) -= debt.1;
        }

        for result in &results {
            let lender_account = accounts
                .iter()
                .find(|acc| acc.id == *result.0)
                .unwrap_or(account);

            // lender will have their own costs (to see the general distribution of cost, so ignore them here
            if lender_account.id == account.id {
                continue;
            }

            #[allow(clippy::cast_precision_loss)]
            all_debts.push(dto::CalculatedDebtDto {
                payer_account: account.clone().into(),
                lender_account: lender_account.clone().into(),
                // only transform to float at the end to not run into rounding errors
                amount: (*result.1 as f64) / 100.0,
            });
        }
    }

    Ok(all_debts)
}
