use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize)]
pub struct AuthRequestQuery {
    pub code: String,
    pub state: String,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Deserialize, IntoParams)]
pub struct AuthRequestParams {
    pub origin_uri: String,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub avatar: Option<String>,
    pub username: String,
    pub discriminator: String,
}

impl AuthUser {
    pub fn account_name(&self) -> String {
        format!("{}#{}", self.username, self.discriminator)
    }
}
