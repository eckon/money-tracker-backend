use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::controller;
use crate::model::dto;

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        dto::AccountDto,
        dto::CalculatedDebtDto,
        dto::CostDto,
        dto::CreateAccountDto,
        dto::CreateCostDto,
        dto::CreatePaymentDto,
        dto::DebtorDto,
        dto::PaymentDto,
    ),),
    paths(
        controller::account::create_account,
        controller::account::get_account,
        controller::account::get_account_tags,
        controller::account::get_all_accounts,
        controller::cost::create_cost,
        controller::cost::get_all_costs,
        controller::cost::get_current_snapshot,
        controller::payment::create_payment,
        controller::payment::get_all_payment,
    )
)]
struct ApiDoc;

#[allow(clippy::expect_used)]
fn generate_docs() -> openapi::OpenApi {
    let docs = ApiDoc::openapi();
    tracing::debug!(
        "{}",
        docs.to_pretty_json().expect("generated docs to exist")
    );
    docs
}

pub fn app(uri: &str) -> SwaggerUi {
    SwaggerUi::new(format!("/{uri}/*tail")).url("/api-doc/openapi.json", generate_docs())
}
