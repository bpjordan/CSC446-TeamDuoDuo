
use rocket::http::Status;
use rocket::futures::{TryStreamExt, TryFutureExt};
use rocket::serde::Serializer;
use rocket::serde::{json::Json, Serialize};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::{FromRow, self};
use sqlx::types::chrono::{DateTime, Local, Utc};

use crate::{auth, db};

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct LogEntry {

    #[serde(serialize_with = "serialize_dt")]
    pub timestamp: DateTime<Utc>,
    pub username_provided: String,
    pub password_provided: String,
    pub success: bool,
    pub user_found: Option<String>,
    pub session_len: Option<u32>,
    pub error: Option<String>
}

// Helper function to serialize the datetime
pub fn serialize_dt<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    dt.with_timezone(&Local)
        .to_rfc3339()
        .serialize(serializer)
}


#[get("/logs?<last>")]
pub async fn query_logs (last: Option<u64>, _s: auth::GymLeaderSession, mut db: Connection<db::Users>)
-> Result<Json<Vec<LogEntry>>, Status> {


    let logs = sqlx::query_as("SELECT * FROM access_log ORDER BY timestamp LIMIT ?")
    .bind(last.unwrap_or(std::u64::MAX))
    .fetch(&mut *db)
    .try_collect::<Vec<_>>()
    .or_else(|e| async move {
        println!("   !! Got error {e}");
        Err(Status::InternalServerError)})
    .await?;

    Ok(Json(logs))

}