
use std::fmt::Debug;

use rocket::futures::TryFutureExt;
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
    pub sprite: String,
    pub image: String,
}

#[get("/users")]
pub async fn query_users(_s: auth::GymLeaderSession ,mut db: Connection<db::Users>) -> Result<Json<Vec<User>>, Status> {

    // Incredibly stupid hacky workaround because Rocket uses an old version of sqlx
    let users = sqlx::query_as(" SELECT users.*, CAST(role AS UNSIGNED)-1 AS role FROM users ")
    .fetch(&mut *db)
    .try_collect::<Vec<_>>().await
    .or_else(|e| {
        println!("   !! Got error {e}");
        Err(Status::InternalServerError)
    })?;

    Ok(Json(users))
}


#[get("/user")]
pub async fn query_current_user(user_session: auth::TrainerSession, mut db: Connection<db::Users>) -> Result<Json<User>, Status> {


    let user = sqlx::query_as(" SELECT users.*, CAST(role AS UNSIGNED)-1 AS role FROM users WHERE username = ?")
    .bind(&user_session.user)
    .fetch_optional(&mut *db)
    .or_else(|e| async move {
        println!("   !! Got error {e}");
        Err(Status::InternalServerError)
    })
    .and_then(|u| async {
        Ok(u.ok_or(Status::NotFound)?)
    }).await?;

    Ok(Json(user))
}