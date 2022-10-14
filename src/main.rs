use std::net::SocketAddr;

use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;

mod api;
mod db;
mod model;

#[tokio::main]
async fn main() -> () {
    dotenv::dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug")
    }

    tracing_subscriber::fmt::init();

    let db_connection_str = std::env::var("DATABASE_URL").expect(".env has valid DATABASE_URL");

    let pool = PgPoolOptions::new()
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");

    let app = Router::new()
        .merge(api::get_router())
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
