
use rocket::{http::Status, serde::{json::Json, Serialize}, futures::TryStreamExt};
use rocket_db_pools::Connection;
use sqlx::FromRow;

use crate::{auth, db};

#[derive(Debug, Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Pokemon {
    name: String,
    r#type: String,
    trainer: String,
    sprite: String,
    image: String
}

#[get("/pokemon")]
pub async fn query_pokemon(_s: auth::ProfessorSession, mut db: Connection<db::Users>) -> Result<Json<Vec<Pokemon>>, Status> {

    let pokemon = sqlx::query_as(" SELECT * FROM pokemon")
    .fetch(&mut *db)
    .try_collect::<Vec<_>>().await
    .or_else(|e| {
        println!("   !! Got error {e}");
        Err(Status::InternalServerError)
    })?;

    Ok(Json(pokemon))

}

#[get("/user_pokemon")]
pub async fn query_user_pokemon(session: auth::TrainerSession, mut db: Connection<db::Users>) -> Result<Json<Vec<Pokemon>>, Status> {

    let pokemon = sqlx::query_as(" SELECT * FROM pokemon WHERE trainer = ?")
    .bind(&session.user)
    .fetch(&mut *db)
    .try_collect::<Vec<_>>().await
    .or_else(|e| {
        println!("   !! Got error {e}");
        Err(Status::InternalServerError)
    })?;

    Ok(Json(pokemon))
}