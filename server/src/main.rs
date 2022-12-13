#[macro_use] extern crate rocket;

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
async fn login_user(mut db: Connection<SiteDB>, username: &str, password: &str) -> Status {

    let user = sqlx::query("SELECT username FROM users WHERE username = ? AND password = ?").bind(username).bind(password)
    .fetch_optional(&mut *db).await;

    match user {
        Ok(Some(_)) => Status::Ok,
        Ok(None) => Status::Unauthorized,
        Err(_) => Status::InternalServerError,
    }

}

#[launch]
fn rocket() -> _ {

    rocket::build()
    .attach(SiteDB::init())
    .mount("/", FileServer::from("/app/static"))
    .mount("/api", routes!(raw_query, login_user))
}
