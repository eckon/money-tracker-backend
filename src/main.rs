use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

mod db;
mod model;

#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user = db::get_user(pool.as_ref(), user_id.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/user/create/{user_name}")]
async fn add_user(
    pool: web::Data<PgPool>,
    user_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user = db::create_user(pool.as_ref(), user_name.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

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
            .service(add_user)
            .service(get_user)
    })
    .bind(server_address)?
    .run()
    .await
}
