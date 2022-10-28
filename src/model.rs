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
    pub entry: Vec<AccountEntryDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountEntryDto {
    pub kind: AccountEntryKind,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountEntryDto {
    pub id: Uuid,
    pub kind: AccountEntryKind,
    pub amount: f64,
}

impl From<AccountEntry> for AccountEntryDto {
    fn from(entry: AccountEntry) -> Self {
        Self {
            amount: (entry.amount as f64) / 100.0,
            id: entry.id,
            kind: entry.kind,
        }
    }
}

impl From<Account> for AccountDto {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            name: account.name,
            entry: vec![],
        }
    }
}

// DB

#[derive(Debug, Deserialize, Serialize)]
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
}
