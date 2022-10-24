use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccount {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}

// TODO: these are upper case for the backend and the api, maybe have a look how to keep them also
// snake case in other envs?
#[derive(sqlx::Type, Debug, Deserialize, Serialize)]
#[sqlx(type_name= "account_entry_kind", rename_all = "snake_case")]
pub enum AccountEntryKind {
    Cost,
    Payment,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccountEntry {
    pub kind: AccountEntryKind,
    // TODO: needs value etc later on
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountEntry {
    pub id: Uuid,
    pub account_id: Uuid,
    pub kind: AccountEntryKind,
    // TODO: needs value etc later on
}
