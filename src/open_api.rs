use utoipa::{openapi, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use crate::controller::{account, cost, payment};
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
        account::create_account,
        account::delete_account,
        account::get_account,
        account::get_account_tags,
        account::get_all_accounts,
        cost::create_cost,
        cost::delete_cost,
        cost::get_all_costs,
        cost::get_current_snapshot,
        payment::create_payment,
        payment::delete_payment,
        payment::get_all_payment,
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
