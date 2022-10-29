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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    #[allow(clippy::expect_used)]
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server can bind to address and serve endpoints");
}
