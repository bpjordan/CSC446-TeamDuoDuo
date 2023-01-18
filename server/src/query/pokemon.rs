
use rocket::{http::Status, serde::{json::Json, Serialize}};
use rocket_db_pools::Connection;

use crate::{auth, db};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Pokemon {}

#[get("/pokemon")]
pub async fn query_pokemon(_s: auth::ProfessorSession, _db: Connection<db::Users>) -> Result<Json<Vec<Pokemon>>, Status> {

    Err(Status::NotImplemented)
}

#[get("/user_pokemon")]
pub async fn query_user_pokemon(_user_session: auth::UserSession, _db: Connection<db::Users>) -> Result<Json<Vec<Pokemon>>, Status> {

    Err(Status::NotImplemented)
}