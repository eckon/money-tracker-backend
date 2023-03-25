use std::collections::{HashMap, HashSet};

use sqlx::MySqlPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::helper::Conversion;
use crate::model::{
    dto::{request, response},
    entity,
};
use crate::service;

pub async fn create(
    pool: &MySqlPool,
    account_id: String,
    debtors: Vec<request::CreateDebtorDto>,
    amount: f64,
    description: Option<String>,
    event_date: chrono::NaiveDate,
    tags: Option<Vec<String>>,
) -> Result<entity::Cost, AppError> {
    struct CreateDebtor {
        account_id: String,
        amount: i64,
    }

    let debtors = debtors
        .iter()
        .map(|d| CreateDebtor {
            account_id: d.account_id.clone(),
            amount: Conversion::to_int(d.amount),
        })
        .collect::<Vec<_>>();

    let amount = Conversion::to_int(amount);
    let debtors_amount_sum = debtors.iter().map(|d| d.amount).sum::<i64>();
    if debtors_amount_sum != amount {
        return Err(AppError::Service(format!(
            "sum of all debtors amount needs to be {amount} but is {debtors_amount_sum}"
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
                    (?,         ?,     ?,          ?,         ?,   ?)
        "#,
        &cost_uuid.to_string(),
        account_id,
        amount,
        description,
        event_date,
        serde_json::json!(&tags)
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
                        (?,                ?,      ?,     ?)
            "#,
            &debt_uuid.to_string(),
            debtor.account_id.to_string(),
            &cost_uuid.to_string(),
            debtor.amount,
        )
        .execute(pool)
        .await?;
    }

    get(pool, cost_uuid.to_string()).await
}

pub async fn delete(pool: &MySqlPool, cost_id: String) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"
            DELETE
                FROM cost
                    WHERE id = ?
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

pub async fn get(pool: &MySqlPool, cost_id: String) -> Result<entity::Cost, AppError> {
    Ok(sqlx::query_as!(
        entity::Cost,
        r#"
            SELECT *
            FROM cost
                WHERE id = ?
        "#,
        cost_id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_for_account(
    pool: &MySqlPool,
    account_id: String,
) -> Result<Vec<entity::Cost>, AppError> {
    Ok(sqlx::query_as!(
        entity::Cost,
        r#"
            SELECT *
            FROM cost
                WHERE account_id = ?
        "#,
        account_id
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_all(
    pool: &MySqlPool,
    start_date: Option<chrono::NaiveDate>,
    end_date: Option<chrono::NaiveDate>,
) -> Result<Vec<response::CostDto>, AppError> {
    struct CostJoinedDebt {
        cost: entity::Cost,
        debt: entity::Debt,
    }

    #[allow(clippy::unwrap_used)]
    let result = sqlx::query!(
        r#"
            SELECT c.*, d.id AS debt_id, d.debtor_account_id, d.amount AS debtor_amount
            FROM cost c
                JOIN debt d ON d.cost_id = c.id
            WHERE
                c.event_date BETWEEN ? AND ?
        "#,
        start_date.unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
        end_date.unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(3000, 1, 1).unwrap()),
    )
    .map(|row| CostJoinedDebt {
        cost: entity::Cost {
            id: row.id.clone(),
            account_id: row.account_id,
            amount: row.amount,
            event_date: row.event_date,
            description: row.description,
            tags: row.tags,
        },
        debt: entity::Debt {
            id: row.debt_id,
            debtor_account_id: row.debtor_account_id,
            cost_id: row.id.clone(),
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
    pool: &MySqlPool,
    account_id: String,
) -> Result<Vec<(String, i64)>, AppError> {
    let records = sqlx::query!(
        r#"
            SELECT d.amount, d.debtor_account_id
                FROM debt d
                    JOIN cost c ON c.id = d.cost_id
                WHERE c.account_id = ?
        "#,
        account_id
    )
    .fetch_all(pool)
    .await?;

    // calculate the overall debt to the different accounts
    let mut results: HashMap<String, i64> = HashMap::new();
    for record in &records {
        *results.entry(record.debtor_account_id.clone()).or_insert(0) += record.amount;
    }

    // transform hashmap into vector
    Ok(results
        .iter()
        .map(|r| (r.0.clone(), *r.1))
        .collect::<Vec<_>>())
}

pub async fn get_debts_for_account(
    pool: &MySqlPool,
    account_id: String,
) -> Result<Vec<(String, i64)>, AppError> {
    let records = sqlx::query!(
        r#"
            SELECT d.amount, c.account_id
                FROM debt d
                    JOIN cost c ON c.id = d.cost_id
                WHERE d.debtor_account_id = ?
        "#,
        account_id
    )
    .fetch_all(pool)
    .await?;

    // calculate the overall debt to the different accounts
    let mut results: HashMap<String, i64> = HashMap::new();
    for record in &records {
        *results.entry(record.account_id.clone()).or_insert(0) += record.amount;
    }

    // transform hashmap into vector
    Ok(results
        .iter()
        .map(|r| (r.0.clone(), *r.1))
        .collect::<Vec<_>>())
}

pub async fn get_current_snapshot(
    pool: &MySqlPool,
) -> Result<Vec<response::CalculatedDebtDto>, AppError> {
    let accounts = service::account::get_all(pool).await?;

    let mut all_debts: Vec<response::CalculatedDebtDto> = Vec::new();
    for account in &accounts {
        let payed_payments = service::payment::get_for_account(pool, account.id.clone()).await?;
        let given_payments = service::payment::get_of_account(pool, account.id.clone()).await?;
        let to_pay_debts = get_debts_for_account(pool, account.id.clone()).await?;
        let being_payed_debts = get_debts_of_account(pool, account.id.clone()).await?;

        all_debts = accumulate_costs(
            &payed_payments,
            &given_payments,
            &to_pay_debts,
            &being_payed_debts,
            &accounts,
            account,
            all_debts,
        );
    }

    Ok(all_debts)
}

fn accumulate_costs(
    payed_payments: &[entity::Payment],
    given_payments: &[entity::Payment],
    to_pay: &[(String, i64)],
    being_paid: &[(String, i64)],
    accounts: &[entity::Account],
    payer_account: &entity::Account,
    accumulated_debts: Vec<response::CalculatedDebtDto>,
) -> Vec<response::CalculatedDebtDto> {
    // calculate the overall debt from payer account to lender account
    let mut results: HashMap<String, i64> = HashMap::new();

    // payer account pays to lender account via payment
    for payment in payed_payments {
        *results
            .entry(payment.lender_account_id.clone())
            .or_insert(0) += payment.amount;
    }

    // lender account could have payed to payer account via payment
    for payment in given_payments {
        *results.entry(payment.payer_account_id.clone()).or_insert(0) -= payment.amount;
    }

    // lender account could have debts to payer account via debts
    for debt in being_paid {
        *results.entry(debt.0.clone()).or_insert(0) += debt.1;
    }

    // payer account could have debts to lender account via debts
    for debt in to_pay {
        *results.entry(debt.0.clone()).or_insert(0) -= debt.1;
    }

    let mut accumulated_debts = accumulated_debts;
    for result in &results {
        let lender_account = accounts
            .iter()
            .find(|acc| acc.id == *result.0)
            .unwrap_or(payer_account);

        // lender will have their own costs (to see the general distribution of cost, so ignore them here
        if lender_account.id == payer_account.id {
            continue;
        }

        accumulated_debts.push(response::CalculatedDebtDto {
            payer_account: payer_account.clone().into(),
            lender_account: lender_account.clone().into(),
            // only transform to float at the end to not run into rounding errors
            amount: Conversion::to_float(*result.1),
        });
    }

    accumulated_debts.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_calculate_current_snapshot() {
        let lender_id = Uuid::new_v4().to_string();
        let payer_id = Uuid::new_v4().to_string();

        let lender_account = entity::Account {
            id: lender_id.clone(),
            name: "Lender".to_string(),
        };

        let payer_account = entity::Account {
            id: payer_id.clone(),
            name: "Payer".to_string(),
        };

        // payer added cost of 412
        //   with a split of lender needs to pay 11, rest is payed by payer (and ignored - as cant ow yourself)
        let to_pay = vec![(lender_id.clone(), 11)];
        let being_paid = vec![(payer_id.clone(), 401)];

        // lender pays back 100 to payer
        let payed_payments = vec![entity::Payment {
            id: Uuid::new_v4().to_string(),
            payer_account_id: payer_id,
            lender_account_id: lender_id,
            amount: 100,
            event_date: chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            description: None,
        }];

        // noone pays back to lender
        let given_payments = vec![];

        let results = accumulate_costs(
            &payed_payments,
            &given_payments,
            &to_pay,
            &being_paid,
            &[lender_account, payer_account.clone()],
            &payer_account,
            vec![],
        );

        // resulting in lender: 11 was needed, 100 was payed, so 89 is now owed to lender instead
        assert_eq!(results[0].amount, 0.89);
    }
}
