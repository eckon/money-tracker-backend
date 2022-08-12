use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/user/{user_id}")]
async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let user_uid = user_id.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::find_user_by_uid(&mut conn, user_uid)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {user_uid}"));
        Ok(res)
    }
}

// for testing just a get route
#[get("/user-create")]
async fn add_user(
    pool: web::Data<DbPool>,
    // form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        // actions::insert_new_user(&mut conn, &form.name)
        actions::insert_new_user(&mut conn, "Me")
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

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
