use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Payment {
    pub id: String,
    pub payer_account_id: String,
    pub lender_account_id: String,
    pub amount: i64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Debt {
    pub id: String,
    pub debtor_account_id: String,
    pub cost_id: String,
    pub amount: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cost {
    pub id: String,
    pub account_id: String,
    pub amount: i64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
    pub tags: Option<Value>,
}
