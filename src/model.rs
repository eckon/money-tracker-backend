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

impl From<Account> for AccountDto {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            name: account.name,
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
    pub lender_account_id: Uuid, // mayube rename to payback?
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
