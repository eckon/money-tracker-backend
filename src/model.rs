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
