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
    pub entries: Option<Vec<AccountEntryDto>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountEntryDto {
    pub kind: AccountEntryKind,
    pub amount: f64,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub event_date: chrono::NaiveDate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountEntryDto {
    pub id: Uuid,
    pub kind: AccountEntryKind,
    pub amount: f64,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub event_date: chrono::NaiveDate,
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

impl From<AccountEntry> for AccountEntryDto {
    fn from(entry: AccountEntry) -> Self {
        Self {
            amount: (entry.amount as f64) / 100.0,
            id: entry.id,
            kind: entry.kind,
            description: entry.description,
            tags: entry.tags,
            event_date: entry.event_date,
        }
    }
}

impl From<Account> for AccountDto {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            name: account.name,
            entries: None,
        }
    }
}

// DB

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone)]
#[sqlx(type_name = "account_entry_kind", rename_all = "snake_case")]
pub enum AccountEntryKind {
    Cost,
    Payment,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountEntry {
    pub id: Uuid,
    pub account_id: Uuid,
    pub kind: AccountEntryKind,
    pub amount: i64,
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
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
