
use std::fmt::Debug;

use rocket::http::Status;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::{self, FromRow};
use rocket::futures::stream::TryStreamExt;
use sqlx::Decode;

use crate::auth;
use crate::db;

// Model of a user in the database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Decode)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: auth::UserRole,
    pub session: Option<String>,
}

#[get("/users")]
pub async fn query_users(_user_session: auth::UserSession ,mut db: Connection<db::Users>) -> Result<Json<Vec<User>>, Status> {

    let users = sqlx::query_as("SELECT * FROM users")
    .fetch(&mut *db)
    .try_collect::<Vec<_>>().await
    .or(Err(Status::InternalServerError))?;

    Ok(Json(users))
}