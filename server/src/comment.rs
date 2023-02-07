use rocket::form::Form;
use rocket::http::Status;
use rocket::{fairing::AdHoc, serde::Serialize};
use rocket::futures::{TryStreamExt, TryFutureExt};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

use crate::{db, query::log::serialize_dt};

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Comment {
    #[serde(serialize_with = "serialize_dt")]
    pub timestamp: DateTime<Utc>,
    pub content: String,
}

#[derive(Debug, FromForm)]
pub struct NewComment {
    content: String,
}

// Endpoint that returns comments
#[get("/query")]
async fn query_comments(mut db: Connection<db::Users>) -> Result<Json<Vec<Comment>>, Status> {
    let comments: Vec<Comment> = sqlx::query_as(
        "SELECT content, timestamp FROM comment ORDER BY timestamp"
    )
    .fetch(&mut *db)
    .try_collect::<Vec<_>>()
    .or_else(|e| async move {
        println!("   !! Got error {e}");
        Err(Status::InternalServerError)})
    .await?;

    Ok(Json(comments))
}

// Endpoint to post comments
#[put("/add_comment", data="<comment>")]
async fn add_comment(comment: Form<NewComment>, db_pool: &db::Users) -> Result<(), Status> {
    
    // Read the comment from the Form
    let NewComment { content: _content } = comment.into_inner();
    
    // Insert into the database
    sqlx::query(
        "INSERT INTO comment(
            content
        )
        VALUES (?)"
    )
    .bind(_content)
    .execute(&mut db_pool.acquire().await.or(Err(Status::ServiceUnavailable))?)
    .await
    .or_else(|e| {
        eprintln!("   !! Couldn't insert log: {e}");
        Err(Status::InternalServerError)
    })?;

    // Success!
    Ok(())
}

// Function called by main to add module to the api
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Blog endpoints", |rocket| async {
        rocket
        .mount("/api/blog", routes![
            query_comments, add_comment])
    })
}