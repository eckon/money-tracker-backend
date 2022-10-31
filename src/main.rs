use std::net::SocketAddr;

use axum::{middleware, Extension, Router};
use hyper::Method;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

mod api;
mod logging;
mod model;
mod service;

#[allow(clippy::expect_used)]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }

    tracing_subscriber::fmt::init();

    let db_connection_str = std::env::var("DATABASE_URL").expect(".env has valid DATABASE_URL");

    let pool = PgPoolOptions::new()
        .connect(&db_connection_str)
        .await
        .expect("pool can connect to database");

    let app = Router::new()
        .merge(api::app())
        .layer(Extension(pool))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(logging::print_request_response));

    let api_config_str = std::env::var("API_ADDR").expect(".env has valid API_ADDR");
    let api_config = parse_api_config(&api_config_str);
    let addr = SocketAddr::from(api_config);
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server can bind to address and serve endpoints");
}

#[allow(clippy::expect_used)]
fn parse_api_config(config_string: &str) -> ([u8; 4], u16) {
    let cfg = config_string
        .split(':')
        .collect::<Vec<&str>>()
        .iter()
        .flat_map(|str| str.split('.'))
        .map(|str| {
            str.parse::<u16>()
                .expect("API_ADDR can be parsed as unsigned numbers")
        })
        .collect::<Vec<u16>>();

    let api_port = cfg[4];

    #[allow(clippy::cast_possible_truncation)]
    let api_ip = cfg[0..4]
        .iter()
        .copied()
        .map(|digit| digit as u8)
        .collect::<Vec<u8>>()
        .try_into()
        .expect("API_ADDR ip part can be parsed");

    (api_ip, api_port)
}
