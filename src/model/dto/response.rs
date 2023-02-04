use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{conversion::Conversion, model::entity};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct AccountDto {
    pub id: Uuid,
    pub name: String,
}

impl From<entity::Account> for AccountDto {
    fn from(account: entity::Account) -> Self {
        Self {
            id: account.id,
            name: account.name,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct PaymentDto {
    pub id: Uuid,
    pub payer_account_id: Uuid,
    pub lender_account_id: Uuid,
    pub amount: f64,

    #[schema(value_type = String)]
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
}

impl From<entity::Payment> for PaymentDto {
    fn from(payment: entity::Payment) -> Self {
        Self {
            id: payment.id,
            amount: Conversion::to_float(payment.amount),
            payer_account_id: payment.payer_account_id,
            lender_account_id: payment.lender_account_id,
            event_date: payment.event_date,
            description: payment.description,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct CalculatedDebtDto {
    pub payer_account: AccountDto,
    pub lender_account: AccountDto,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct DebtDto {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: f64,
}

impl From<entity::Debt> for DebtDto {
    fn from(cost: entity::Debt) -> Self {
        Self {
            id: cost.id,
            account_id: cost.debtor_account_id,
            amount: Conversion::to_float(cost.amount),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct CostDto {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: f64,
    pub debtors: Vec<DebtDto>,

    #[schema(value_type = String)]
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl From<entity::Cost> for CostDto {
    fn from(cost: entity::Cost) -> Self {
        Self {
            id: cost.id,
            tags: cost.tags,
            amount: Conversion::to_float(cost.amount),
            debtors: Vec::new(),
            account_id: cost.account_id,
            event_date: cost.event_date,
            description: cost.description,
        }
    }
}
