
use std::fmt::Debug;

use rocket::http::Status;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::mysql::MySqlRow;
use rocket_db_pools::sqlx::{self, Row, FromRow};

use rocket::futures::stream::TryStreamExt;

use crate::auth;
use crate::db;

// Model of a user in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: auth::UserRole,
    pub session: Option<String>,
}

impl FromRow<'_, MySqlRow> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            email: row.try_get("email")?,
            role: row.try_get::<String, _>("role")?
                    .parse().or(Err(sqlx::Error::Decode("not a role".into())))?,
            session: row.try_get("session")?
        })
    }
}

#[get("/users")]
pub async fn query_users(_user_session: auth::UserSession ,mut db: Connection<db::Users>) -> Result<Json<Vec<User>>, Status> {

    let users = sqlx::query_as("SELECT * FROM users")
    .fetch(&mut *db)
    .try_collect::<Vec<_>>().await
    .or(Err(Status::InternalServerError))?;

    Ok(Json(users))
}