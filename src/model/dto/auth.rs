use axum::extract::FromRequest;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use async_session::{MemoryStore, SessionStore};
use axum::{
    async_trait,
    extract::{RequestParts, TypedHeader},
    Extension,
};
use headers::{authorization::Bearer, Authorization};

use crate::error::AppError;

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

#[async_trait]
impl<S> FromRequest<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<MemoryStore>::from_request(req)
            .await
            .map_err(|err| AppError::InternalServer(err.to_string()))?;

        let bearer = match TypedHeader::<Authorization<Bearer>>::from_request(req).await {
            Ok(TypedHeader(Authorization(bearer))) => bearer,
            Err(_) => return Err(AppError::Forbidden),
        };

        let session = store
            .load_session(bearer.token().to_string())
            .await
            .unwrap_or(None)
            .ok_or(AppError::Forbidden)?;

        let user = session.get::<Self>("user").ok_or(AppError::Forbidden)?;

        // TODO: this is a quickfix until correct user accounts are implemented via db
        ["eckon#5962", "Hanawa#5326"]
            .iter()
            .any(|acc| *(*acc).to_string() == user.account_name())
            .then_some(0)
            .ok_or(AppError::Forbidden)?;

        Ok(user)
    }
}
