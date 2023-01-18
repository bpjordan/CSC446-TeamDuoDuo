
use rocket::fairing::AdHoc;
use rocket_db_pools::{Database, sqlx};


#[derive(Database)]
#[database("maindb")]
pub struct Users(sqlx::MySqlPool);

// Function called by main to add module to the api
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database", |rocket| async {
        rocket.attach(Users::init())
    })
}