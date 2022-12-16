#[macro_use] extern crate rocket;
extern crate argon2;

use std::fmt::Debug;

use argon2::{Argon2, PasswordVerifier, PasswordHash, password_hash::Error::Password};

use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

use rocket::futures::stream::TryStreamExt;

/// Creates a closure to log an error and transform it to a 500 status code.
/// 
/// Meant to be used as an argument to `Result::map_err`.
/// 
/// Example:  
/// ```rust,no_run
/// use std::fs::read_to_string;
/// s = read_to_string("doc.txt").map_err(log_server_err("reading doc.txt"));
/// assert!(s, Status::InternalServerError);
/// ```
/// 
/// Output:
/// ```text
///    !! Error reading doc.txt: Not Found
/// ```
// 
fn log_server_err<F: Debug>(process: &'static str) -> impl FnOnce(F) -> Status {
    move |e| {
        println!("   !! Failed to {}: {:?}", process, e);
        Status::InternalServerError
    }
}

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
async fn raw_query(mut db: Connection<SiteDB>) -> Result<Json<Vec<User>>, Status> {

    let users = sqlx::query("SELECT * FROM users;")
    .fetch(&mut *db)
    .map_ok(|r| User{username: r.get("username"), password: r.get("password"), email: r.get("email") })
    .try_collect::<Vec<_>>().await
    .map_err(log_server_err("reading from database"))?;

    Ok(Json(users))
}

#[get("/")]
fn redirect_to_login() -> Redirect {
    Redirect::to(uri!("/login.html"))
}

#[post("/login?<username>&<password>")]
async fn login_user(mut db: Connection<SiteDB>, username: &str, password: &str) -> Result<Status, Status> {

    let hashslinginghasher = Argon2::default();

    let entry = sqlx::query("SELECT password FROM users WHERE username = ?").bind(username)
    .fetch_optional(&mut *db).await.map_err(log_server_err("fetching user from database"))?;

    let pwhash = match &entry {
        Some(row) => row.get(0),
        None => return Ok(Status::Unauthorized),
    };

    let hash_result = hashslinginghasher.verify_password(
        password.as_bytes(),
        &PasswordHash::new(pwhash).map_err(log_server_err("parsing hash from database"))?
    );

    match hash_result {
        Ok(()) => Ok(Status::Ok),
        Err(e) => match e {
            Password => Ok(Status::Unauthorized),
            _ => Err(Status::InternalServerError)
        },

    }

}

#[launch]
fn rocket() -> _ {

    rocket::build()
    .attach(SiteDB::init())
    .mount("/", FileServer::from("/app/static"))
    .mount("/", routes!(redirect_to_login))
    .mount("/api", routes!(raw_query, login_user))
}
