use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::entity;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateAccountDto {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountDto {
    pub id: Uuid,
    pub name: String,
}

impl From<entity::Account> for AccountDto {
    fn from(account: entity::Account) -> Self {
        Self {
            id: account.id,
            name: account.name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreatePaymentDto {
    pub lender_account_id: Uuid,
    pub amount: f64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaymentDto {
    pub id: Uuid,
    pub payer_account_id: Uuid,
    pub lender_account_id: Uuid,
    pub amount: f64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
}

impl From<entity::Payment> for PaymentDto {
    fn from(payment: entity::Payment) -> Self {
        Self {
            id: payment.id,
            amount: (payment.amount as f64) / 100.0,
            payer_account_id: payment.payer_account_id,
            lender_account_id: payment.lender_account_id,
            event_date: payment.event_date,
            description: payment.description,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DebtorDto {
    pub account_id: Uuid,
    pub percentage: i16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CalculatedDebtDto {
    pub payer_account: AccountDto,
    pub lender_account: AccountDto,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateCostDto {
    pub debtors: Vec<DebtorDto>,
    pub amount: f64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CostDto {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: f64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl From<entity::Cost> for CostDto {
    fn from(cost: entity::Cost) -> Self {
        Self {
            id: cost.id,
            tags: cost.tags,
            amount: (cost.amount as f64) / 100.0,
            account_id: cost.account_id,
            event_date: cost.event_date,
            description: cost.description,
        }
    }
}
