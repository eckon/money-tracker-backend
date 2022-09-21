use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod db;
mod model;
mod api;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url.clone())
        .await
        .expect("pool failed");

    let server_address = ("127.0.0.1", 3000);
    log::info!(
        "Starting HTTP server on {}:{}",
        server_address.0,
        server_address.1
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(api::add_user)
            .service(api::get_user)
    })
    .bind(server_address)?
    .run()
    .await
}
