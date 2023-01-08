use std::collections::{HashMap, HashSet};

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::{
    dto::{request, response},
    entity,
};
use crate::service;

pub async fn create(
    pool: &PgPool,
    account_id: Uuid,
    debtors: Vec<request::CreateDebtorDto>,
    amount: f64,
    description: Option<String>,
    event_date: chrono::NaiveDate,
    tags: Option<Vec<String>>,
) -> Result<entity::Cost, AppError> {
    struct CreateDebtor {
        account_id: Uuid,
        amount: i64,
    }

    #[allow(clippy::cast_possible_truncation)]
    let debtors = debtors
        .iter()
        .map(|d| CreateDebtor {
            account_id: d.account_id,
            amount: (d.amount * 100.0) as i64,
        })
        .collect::<Vec<_>>();

    #[allow(clippy::cast_possible_truncation)]
    let amount: i64 = (amount * 100.0) as i64;
    let debtors_amount_sum = debtors.iter().map(|d| d.amount).sum::<i64>();
    if debtors_amount_sum != amount {
        return Err(AppError::Service(format!(
            "sum of all debtors amount needs to be {} but is {}",
            amount, debtors_amount_sum
        )));
    }

    // sort and remove duplicate values
    let tags = tags
        .unwrap_or_default()
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    let cost_uuid = Uuid::new_v4();
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
                        (id, debtor_account_id, cost_id, amount)
                    VALUES
                        ($1,                $2,      $3,     $4)
            "#,
            &debt_uuid,
            debtor.account_id,
            &cost_uuid,
            debtor.amount,
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

pub async fn get_all(pool: &PgPool) -> Result<Vec<response::CostDto>, AppError> {
    struct CostJoinedDebt {
        cost: entity::Cost,
        debt: entity::Debt,
    }

    let result = sqlx::query!(
        r#"
            SELECT c.*, d.id AS debt_id, d.debtor_account_id, d.amount AS debtor_amount
            FROM cost c
                JOIN debt d ON d.cost_id = c.id
        "#,
    )
    .map(|row| CostJoinedDebt {
        cost: entity::Cost {
            id: row.id,
            account_id: row.account_id,
            amount: row.amount,
            event_date: row.event_date,
            description: row.description,
            tags: row.tags,
        },
        debt: entity::Debt {
            id: row.debt_id,
            debtor_account_id: row.debtor_account_id,
            cost_id: row.id,
            amount: row.debtor_amount,
        },
    })
    .fetch_all(pool)
    .await?
    .iter()
    // group by cost-id as db will return multiple rows for a join and we want it grouped into a vector
    .fold(
        Into::<Vec<response::CostDto>>::into(Vec::new()),
        |mut acc, e| {
            if let Some(entry) = acc.iter_mut().find(|pred| pred.id == e.cost.id) {
                entry.debtors.push(e.debt.clone().into());
            } else {
                let mut cost: response::CostDto = e.cost.clone().into();
                cost.debtors.push(e.debt.clone().into());
                acc.push(cost);
            }

            acc
        },
    );

    Ok(result)
}

pub async fn get_debts_of_account(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<(Uuid, i64)>, AppError> {
    let records = sqlx::query!(
        r#"
            SELECT d.amount, d.debtor_account_id
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
        *results.entry(record.debtor_account_id).or_insert(0) += record.amount;
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
            SELECT d.amount, c.account_id
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
        *results.entry(record.account_id).or_insert(0) += record.amount;
    }

    // transform hashmap into vector
    Ok(results.iter().map(|r| (*r.0, *r.1)).collect::<Vec<_>>())
}

pub async fn get_current_snapshot(
    pool: &PgPool,
) -> Result<Vec<response::CalculatedDebtDto>, AppError> {
    let accounts = service::account::get_all(pool).await?;

    let mut all_debts: Vec<response::CalculatedDebtDto> = Vec::new();
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
            all_debts.push(response::CalculatedDebtDto {
                payer_account: account.clone().into(),
                lender_account: lender_account.clone().into(),
                // only transform to float at the end to not run into rounding errors
                amount: (*result.1 as f64) / 100.0,
            });
        }
    }

    Ok(all_debts)
}
