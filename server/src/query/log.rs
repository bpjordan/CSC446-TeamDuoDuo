
use rocket::http::Status;
use rocket::futures::{TryStreamExt, TryFutureExt};
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::{FromRow, mysql::MySqlRow, Row, self};
use sqlx::types::chrono::{DateTime, Local};

use crate::{auth, db};

use super::users::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LogEntry {
    pub timestamp: String,
    pub username_provided: String,
    pub password_provided: String,
    pub success: bool,
    pub user_found: Option<User>,
    pub session_len: Option<u32>,
    pub error: Option<String>
}

impl FromRow<'_, MySqlRow> for LogEntry {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, rocket_db_pools::sqlx::Error> {
        Ok(Self {
            timestamp: row.try_get::<DateTime<Local>, _>("timestamp")?
                .to_string(),
            username_provided: row.try_get("username_provided")?,
            password_provided: row.try_get("password_provided")?,
            success: row.try_get("success")?,
            user_found: row.try_get::<Option<String>, _>("user_found")?
                .and_then(|_| {
                    Some(User::from_row(row).ok()?)
                }),
            session_len: row.try_get("session_len")?,
            error: row.try_get("error")?
        })
    }
}

#[get("/logs")]
pub async fn query_logs (_user_session: auth::UserSession, mut db: Connection<db::Users>)
-> Result<Json<Vec<LogEntry>>, Status> {

    let logs = sqlx::query_as("
    SELECT * FROM access_log
    LEFT JOIN users ON access_log.user_found = users.username
    ")
    .fetch(&mut *db)
    .try_collect::<Vec<_>>()
    .or_else(|e| async move {
        println!("   !! Got error {e}");
        Err(Status::InternalServerError)})
    .await?;

    Ok(Json(logs))

}