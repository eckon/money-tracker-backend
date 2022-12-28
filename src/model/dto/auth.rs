use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::SystemTime,
};

use axum::extract::FromRequestParts;
use http::request::Parts;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::IntoParams;

use axum::{async_trait, extract::TypedHeader};
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

    pub fn generate_access_token(&self) -> String {
        // TODO: use some crypt instead of hash or maybe even jwt with refresh token etc
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        SystemTime::now().hash(&mut hasher);
        hasher.finish().to_string()
    }
}

impl Hash for AuthUser {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.avatar.hash(state);
        self.username.hash(state);
        self.discriminator.hash(state);
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AppError::Forbidden)?;

        let pool = parts
            .extensions
            .get::<PgPool>()
            .ok_or(AppError::InternalServer(
                "pool could not be found".to_owned(),
            ))?;

        let auth_user = sqlx::query_as!(
            AuthUser,
            r#"
                SELECT id, avatar, username, discriminator
                FROM auth_user
                    WHERE access_token = $1
            "#,
            &bearer.token().to_string(),
        )
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Forbidden)?;

        // TODO: this is a quickfix until correct user accounts are implemented via db
        ["eckon#5962", "Hanawa#5326"]
            .iter()
            .any(|acc| *(*acc).to_string() == auth_user.account_name())
            .then_some(0)
            .ok_or(AppError::Forbidden)?;

        Ok(auth_user)
    }
}
