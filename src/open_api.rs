use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::api;
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
        api::create_account,
        api::create_cost,
        api::create_payment,
        api::get_account,
        api::get_account_tags,
        api::get_all_accounts,
        api::get_all_costs,
        api::get_all_payment,
        api::get_current_snapshot,
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
