use std::net::SocketAddr;

use axum::{middleware, Extension, Router};
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;

mod api;
mod logging;
mod model;
mod service;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }

    tracing_subscriber::fmt::init();

    #[allow(clippy::expect_used)]
    let db_connection_str = std::env::var("DATABASE_URL").expect(".env has valid DATABASE_URL");

    #[allow(clippy::expect_used)]
    let pool = PgPoolOptions::new()
        .connect(&db_connection_str)
        .await
        .expect("pool can connect to database");

    let app = Router::new()
        .merge(api::app())
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(logging::print_request_response));

    #[allow(clippy::expect_used)]
    let api_config_str = std::env::var("API_ADDR").expect(".env has valid API_ADDR");
    let api_config = parse_api_config(&api_config_str);
    let addr = SocketAddr::from(api_config);
    tracing::debug!("listening on {}", addr);

    #[allow(clippy::expect_used)]
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server can bind to address and serve endpoints");
}

fn parse_api_config(config_string: &str) -> ([u8; 4], u16) {
    #[allow(clippy::expect_used)]
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

    let api_ip = [cfg[0], cfg[1], cfg[2], cfg[3]].map(|digit| digit as u8);
    let api_port = cfg[4];

    (api_ip, api_port)
}
