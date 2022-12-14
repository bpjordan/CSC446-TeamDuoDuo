#[macro_use] extern crate rocket;
extern crate argon2;

use argon2::{Argon2, PasswordVerifier, PasswordHash};

use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

use rocket::futures::stream::TryStreamExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    username: String,
    password: String,
    email: String,
}

#[derive(Database)]
#[database("maindb")]
struct SiteDB(sqlx::MySqlPool);

#[get("/query")]
async fn raw_query(mut db: Connection<SiteDB>) -> Result<Json<Vec<User>>, (Status, &'static str)> {

    let users = sqlx::query("SELECT * FROM users;")
    .fetch(&mut *db)
    .map_ok(|r| User{username: r.get("username"), password: r.get("password"), email: r.get("email") })
    .try_collect::<Vec<_>>()
    .await.map_err(|_| (Status::InternalServerError, "Database query failed"))?;

    Ok(Json(users))
}

#[post("/login?<username>&<password>")]
async fn login_user(mut db: Connection<SiteDB>, username: &str, password: &str) -> Result<Status, Status> {

    if password.len() < 3 { return Ok(Status::BadRequest) };

    let hashslinginghasher = Argon2::default();

    let entry = sqlx::query("SELECT password FROM users WHERE username = ?").bind(username)
    .fetch_optional(&mut *db).await;

    let pwhash = match &entry {
        Ok(Some(row)) => Ok(row.get(0)),
        Ok(None) => Ok("$argon2id$v=19$m=4096,t=3,p=1$7cw5ikPiUevZbFw3Oc67GQ$5BfCIWW7v4pYqqHkaDtF2H7G7TXts9+EX69KW5kCyMA"), //hash for an empty string. This ensures the algorithm is still run to dissuade timing attacks
        Err(_) => Err(Status::InternalServerError),
    }?;

    let hash_result = hashslinginghasher.verify_password(
        password.as_bytes(),
        &PasswordHash::new(pwhash).map_err(|_| Status::InternalServerError)?
    );

    match hash_result {
        Ok(()) => Ok(Status::Ok),
        Err(argon2::password_hash::Error::Password) => Ok(Status::Unauthorized),
        _ => Err(Status::InternalServerError)
    }

}

#[launch]
fn rocket() -> _ {

    rocket::build()
    .attach(SiteDB::init())
    .mount("/", FileServer::from("/app/static"))
    .mount("/api", routes!(raw_query, login_user))
}
