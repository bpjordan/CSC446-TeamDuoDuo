#[macro_use] extern crate rocket;
extern crate argon2;

use std::fmt::Debug;

use argon2::{Argon2, PasswordVerifier, PasswordHash, password_hash::Error::Password};

use auth::generate_jwt;
use rand::RngCore;
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::http::{Status, CookieJar, Cookie};
use rocket::response::Redirect;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

use rocket::futures::stream::TryStreamExt;

mod auth;
mod db;

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
    session: Option<String>,
}

#[get("/query")]
async fn raw_query(_user_session: auth::UserSession ,mut db: Connection<db::Users>) -> Result<Json<Vec<User>>, Status> {

    let users = sqlx::query("SELECT * FROM users;")
    .fetch(&mut *db)
    .map_ok(|r| User{
        username: r.get("username"),
        password: r.get("password"),
        email: r.get("email"),
        session: r.get("session")
    })
    .try_collect::<Vec<_>>().await
    .map_err(log_server_err("reading from database"))?;

    Ok(Json(users))
}

#[get("/query", rank = 2)]
fn query_unauthorized() -> Status {
    Status::Unauthorized
}

#[get("/")]
fn redirect_to_login() -> Redirect {
    Redirect::to(uri!("/login.html"))
}

#[derive(FromForm)]
struct UserCredentials {
    username: String,
    password: String,
}

#[post("/login", data="<creds>")]
async fn login_user(creds: Form<UserCredentials>, cookies: &CookieJar<'_>, mut db: Connection<db::Users>) -> Result<Status, Status> {

    let hashslinginghasher = Argon2::default();

    let entry = sqlx::query("SELECT password FROM users WHERE username = ?").bind(&creds.username)
    .fetch_optional(&mut *db).await.map_err(log_server_err("fetching user from database"))?;

    let pwhash = match &entry {
        Some(row) => row.get(0),
        None => return Err(Status::Unauthorized),
    };

    if let Err(e) = hashslinginghasher.verify_password(
        creds.password.as_bytes(),
        &PasswordHash::new(pwhash).map_err(log_server_err("parsing hash from database"))?
    ) {
        return match e {
            Password => Err(Status::Unauthorized),
            _ => Err(Status::InternalServerError)
        }
    }

    let mut session = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut session);
    let session = hex::encode_upper(session);

    sqlx::query("UPDATE users SET session = ? WHERE username = ?").bind(&session).bind(&creds.username)
    .execute(&mut *db).await.unwrap();

    cookies.add_private(Cookie::new("session_token", generate_jwt(creds.username.clone(), session)));

    Ok(Status::Ok)

}

#[launch]
fn rocket() -> _ {

    rocket::build()
    .attach(db::Users::init())
    .mount("/", FileServer::from("/app/static"))
    .mount("/", routes!(redirect_to_login))
    .mount("/api", routes!(raw_query, query_unauthorized, login_user))
}
