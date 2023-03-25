use axum::{
    extract::{Query, TypedHeader},
    response::Redirect,
    routing::get,
    Extension, Router,
};
use headers::{authorization::Bearer, Authorization};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use sqlx::MySqlPool;

use crate::{
    error::AppError,
    model::dto::auth::{AuthRequestParams, AuthRequestQuery, AuthUser},
};

pub fn app() -> Router {
    Router::new()
        .route("/auth/discord", get(discord_auth))
        .route("/auth/authorized", get(login_authorized))
        .route("/auth/logout", get(logout))
}

#[allow(clippy::expect_used)]
pub fn oauth_client() -> BasicClient {
    let client_id = std::env::var("CLIENT_ID").expect(".env has discord CLIENT_ID");
    let client_secret = std::env::var("CLIENT_SECRET").expect(".env has discord CLIENT_SECRET");
    let redirect_url = std::env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:3000/auth/authorized".to_string());

    let auth_url = std::env::var("AUTH_URL").unwrap_or_else(|_| {
        "https://discord.com/api/oauth2/authorize?response_type=code".to_string()
    });

    let token_url = std::env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).expect("auth url can be created"),
        Some(TokenUrl::new(token_url).expect("token url can be created")),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).expect("redirect url can be created"))
}

/// This call will redirect the user to the discord auth page (triggered by the api)
/// the passed `origin_uri` will tell the backend after discord auth part where to redirect again
///
/// The generated `bearer token` can be found appended at the url via the `access_token` query param
#[utoipa::path(get, path = "/auth/discord", params(AuthRequestParams))]
#[allow(clippy::unused_async)]
async fn discord_auth(
    Extension(client): Extension<BasicClient>,
    Query(query): Query<AuthRequestParams>,
) -> Result<Redirect, AppError> {
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // add the location of the caller (e.g. frontend) for later redirect
        .add_extra_param("state", query.origin_uri)
        // do not prompt user if they already authed to app in the past
        .add_extra_param("prompt", "none")
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Ok(Redirect::to(auth_url.as_ref()))
}

#[utoipa::path(
    get,
    path = "/auth/logout",
    security(("bearer_token" = []))
)]
async fn logout(
    Extension(pool): Extension<MySqlPool>,
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
) -> Result<(), AppError> {
    // delete all logins of given user (not just the given access_token)
    let auth_user = sqlx::query_as!(
        AuthUser,
        r#"
            SELECT id, avatar, username, discriminator
            FROM auth_user
                WHERE access_token = ?
        "#,
        &bearer.token().to_string(),
    )
    .fetch_one(&pool)
    .await?;

    sqlx::query!(
        r#"
            DELETE
            FROM auth_user
                WHERE id = ?
        "#,
        auth_user.id,
    )
    .execute(&pool)
    .await?;

    Ok(())
}

async fn login_authorized(
    Query(query): Query<AuthRequestQuery>,
    Extension(oauth_client): Extension<BasicClient>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Redirect, AppError> {
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await;

    let Ok(token) = token_result else {
        return Err(AppError::Forbidden);
    };

    // Fetch user data from discord
    let client = reqwest::Client::new();
    let user_data: AuthUser = client
        // https://discord.com/developers/docs/resources/user#get-current-user
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .map_err(|_| AppError::Forbidden)?
        .json::<AuthUser>()
        .await
        .map_err(|_| AppError::Forbidden)?;

    let access_token = user_data.generate_access_token();

    sqlx::query!(
        r#"
            INSERT
                INTO auth_user
                    (id, avatar, username, discriminator, access_token)
                VALUES
                    (?,     ?,       ?,            ?,           ?)
        "#,
        user_data.id,
        user_data.avatar,
        user_data.username,
        user_data.discriminator,
        &access_token,
    )
    .execute(&pool)
    .await?;

    // redirect to the given url of the calling party (e.g. frontend)
    // state is kept between calling third party and return, it contains the redirect uri
    let uri = query.state;
    let redirect_url = format!("{uri}?access_token={access_token}");
    Ok(Redirect::temporary(&redirect_url))
}
