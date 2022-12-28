use utoipa::{
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::auth;
use crate::controller::{account, cost, payment};
use crate::model::dto::{request, response};

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        request::CreateAccountDto,
        request::CreateCostDto,
        request::CreateDebtorDto,
        request::CreatePaymentDto,
        response::AccountDto,
        response::CalculatedDebtDto,
        response::CostDto,
        response::DebtDto,
        response::PaymentDto,
    )),
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
        auth::discord_auth,
        auth::logout,
    ),
    modifiers(&SecurityAddon),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_token",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            );
        }
    }
}

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
    SwaggerUi::new(format!("/{uri}")).url("/api-doc/openapi.json", generate_docs())
}
