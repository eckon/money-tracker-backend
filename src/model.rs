use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: split these into dtos etc
// DTO

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountDto {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountDto {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreatePaymentDto {
    pub lender_account_id: Uuid,
    pub amount: f64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateCostDto {
    pub debtor_account_ids: Vec<Uuid>,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaymentDto {
    pub id: Uuid,
    pub payer_account_id: Uuid,
    pub lender_account_id: Uuid,
    pub amount: f64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
}


impl From<Account> for AccountDto {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            name: account.name,
        }
    }
}

impl From<Cost> for CostDto {
    fn from(cost: Cost) -> Self {
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

impl From<Payment> for PaymentDto {
    fn from(payment: Payment) -> Self {
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

// DB

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Payment {
    pub id: Uuid,
    pub payer_account_id: Uuid,
    pub lender_account_id: Uuid,
    pub amount: i64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Debt {
    pub id: Uuid, // might not be needed, as it is just a link between cost and account
    pub debtor_account_id: Uuid,
    pub cost_id: Uuid,
    pub percantage: i8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cost {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: i64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}
