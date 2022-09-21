use sqlx::PgPool;
use uuid::Uuid;

use crate::model;

pub async fn get_user(pool: &PgPool, user_id: Uuid) -> Result<model::User, sqlx::Error> {
    sqlx::query_as!(model::User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await
}

pub async fn create_user(pool: &PgPool, user_name: String) -> Result<model::User, sqlx::Error> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        &uuid,
        user_name,
    )
    .execute(pool)
    .await?;

    get_user(pool, uuid).await
}
