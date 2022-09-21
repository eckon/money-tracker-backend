use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct User {
    id: Uuid,
    name: String,
}

#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let data = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id = $1",
        user_id.into_inner()
    )
    .fetch_one(pool.as_ref())
    .await
    .expect("failed to get data");

    Ok(HttpResponse::Ok().json(data))
}

#[get("/user-create/{user_name}")]
async fn add_user(
    pool: web::Data<PgPool>,
    user_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        Uuid::new_v4(),
        user_name.into_inner(),
    )
    .execute(pool.as_ref())
    .await
    .expect("failed to create data");

    Ok(HttpResponse::Ok().json("ok".to_owned()))
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

    log::info!("Starting HTTP server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(add_user)
            .service(get_user)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
