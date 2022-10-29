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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountEntryDto {
    pub id: Uuid,
    pub kind: AccountEntryKind,
    pub amount: f64,
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
    // not sure how to do it, it needs to be deserialzed i guess
    // already in db just needs to be handled here
    // pub creation_date: Date,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}
