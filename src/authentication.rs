use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    async_trait,
    extract::{FromRequest, Query, RequestParts, TypedHeader},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Extension, Router, Json,
};
use headers::{authorization::Bearer, Authorization};
use http::StatusCode;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

// TODO: this is a quickfix until correct user accounts are implemented via db
static ACCOUNTS_WITH_PERMISSION: &'static [&str] = &["eckon#5962", "Hanawa#5326"];

pub fn app() -> Router {
    Router::new()
        .route("/auth/discord", get(discord_auth))
        .route("/auth/authorized", get(login_authorized))
        .route("/auth/logout", get(logout))
}

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
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub avatar: Option<String>,
    pub username: String,
    pub discriminator: String,
}

impl User {
    fn account_name(&self) -> String {
        format!("{}#{}", self.username, self.discriminator)
    }
}

#[derive(Deserialize)]
struct DiscordParams {
    origin_uri: String,
}

async fn discord_auth(
    Extension(client): Extension<BasicClient>,
    Query(query): Query<DiscordParams>,
) -> impl IntoResponse {
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // add the location of the caller (e.g. frontend) for later redirect
        .add_extra_param("state", query.origin_uri)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Redirect::to(auth_url.as_ref())
}

async fn logout(
    Extension(store): Extension<MemoryStore>,
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let session = match store
        .load_session(bearer.token().to_string())
        .await
        .unwrap()
    {
        Some(s) => s,
        None => return Redirect::to("/"),
    };

    store.destroy_session(session).await.unwrap();
    Redirect::to("/")
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
}

async fn login_authorized(
    Query(query): Query<AuthRequest>,
    Extension(store): Extension<MemoryStore>,
    Extension(oauth_client): Extension<BasicClient>,
) -> impl IntoResponse {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    // Fetch user data from discord
    let client = reqwest::Client::new();
    let user_data: User = client
        // https://discord.com/developers/docs/resources/user#get-current-user
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();

    // Create a new session filled with user data
    let mut session = Session::new();
    session.insert("user", &user_data).unwrap();

    // Store session and get corresponding cookie
    let session_token = store.store_session(session).await.unwrap().unwrap();

    // redirect to the given url of the calling party (e.g. frontend)
    // state is kept between calling third party and return, it contains the redirect uri
    let redirect_url = format!("{}?access_token={}", query.state, session_token);
    Redirect::temporary(&redirect_url).into_response()
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": "no permission",
        }));

        (StatusCode::FORBIDDEN, body).into_response()
    }
}

// TODO: use the impl for my db as well, so i dont need to manually use exntension
#[async_trait]
impl<S> FromRequest<S> for User
where
    S: Send + Sync,
{
    type Rejection = AuthRedirect;

    async fn from_request(req: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<MemoryStore>::from_request(req)
            .await
            .expect("should get in memory store");

        // TODO: this needs to be handled better
        // TODO: names need to be updated
        let Ok(TypedHeader(Authorization(bearer))) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|x| format!("{:?}", x)) else {
                    return Err(AuthRedirect)
                };

        let session = store
            .load_session(bearer.token().to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<Self>("user").ok_or(AuthRedirect)?;

        ACCOUNTS_WITH_PERMISSION
            .iter()
            .any(|acc| acc.to_string() == user.account_name())
            .then_some(0)
            .ok_or(AuthRedirect)?;

        Ok(user)
    }
}
