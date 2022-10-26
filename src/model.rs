use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccount {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}

#[derive(sqlx::Type, Debug, Deserialize, Serialize)]
#[sqlx(type_name = "account_entry_kind", rename_all = "snake_case")]
pub enum AccountEntryKind {
    Cost,
    Payment,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountEntry {
    pub kind: AccountEntryKind,
    pub amount: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountEntry {
    pub id: Uuid,
    pub account_id: Uuid,
    pub kind: AccountEntryKind,
    pub amount: i64,
}
