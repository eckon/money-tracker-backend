use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub id: Uuid,
    pub debtor_account_id: Uuid,
    pub cost_id: Uuid,
    pub percentage: i16,
    pub amount: i64,
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
