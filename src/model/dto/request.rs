use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct CreateAccountDto {
    pub name: String,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct CreatePaymentDto {
    pub lender_account_id: Uuid,
    pub amount: f64,

    #[schema(value_type = String)]
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct DeletePaymentParams {
    pub account_id: Uuid,
    pub payment_id: Uuid,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct CreateCostDto {
    pub debtors: Vec<CreateDebtorDto>,
    pub amount: f64,

    #[schema(value_type = String)]
    pub event_date: chrono::NaiveDate,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct CreateDebtorDto {
    pub account_id: Uuid,
    pub amount: f64,
}

#[derive(Deserialize, IntoParams)]
pub struct DeleteCostParams {
    pub account_id: Uuid,
    pub cost_id: Uuid,
}

#[derive(Deserialize, IntoParams)]
pub struct CostsQuery {
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}
