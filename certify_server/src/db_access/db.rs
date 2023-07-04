use std::f32::consts::E;

use axum::http::StatusCode;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use tracing::info;

use crate::models::{user::{User, RegUser}, error::internal_error};


pub async fn get_user(
    pool: &PgPool,
    email: String,
) -> Result<User, (StatusCode, String)> {
    let users = sqlx::query!(
        "SELECT * FROM users WHERE email = $1",
        email,
    )
    .map({
        |row| User {
            id: row.id,
            user_email: row.email.unwrap_or_default(),
            password_hash: row.password_hash.unwrap_or_default(),
            create_time: NaiveDateTime::from(row.create_time.unwrap()).timestamp_millis(),
        }
    })
    .fetch_one(pool)
    .await
    .map_err(internal_error)?;

    // info!("get_user size: {}", users);

    Ok(users)
}

pub async fn add_new_user_from_db(
    pool: &PgPool,
    user: RegUser,
) -> Result<i32, (StatusCode, String)> {
    println!("add_new_user_from_db user: {}", user.user_email);

    let ts_1970 = NaiveDateTime::from_timestamp_opt(0, 0).unwrap_or_default();

    let insert_result :Result<i32, (StatusCode, String)> = sqlx::query!("INSERT INTO users (email, password_hash, create_time) VALUES ($1, $2, $3) RETURNING id",
        user.user_email,
        user.password_hash,
        ts_1970,
    )
        .map(|row| row.id)
        .fetch_one(pool)
        .await
        .map_err(internal_error);

    match insert_result {
        Ok(user_id) => {
            Ok(user_id)
        }
        Err(e) => Err(e),
    }
}