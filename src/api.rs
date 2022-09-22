use actix_web::{get, web, Error, HttpResponse};

use sqlx::PgPool;
use uuid::Uuid;

use crate::db;

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

pub fn config(cfg: &mut web::ServiceConfig) -> () {
    cfg.service(get_user).service(add_user);
}
