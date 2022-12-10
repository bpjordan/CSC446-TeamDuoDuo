#[macro_use] extern crate rocket;
use rocket::fs::FileServer;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/")]
fn raw_query() -> String {

    "Hello, World".to_string()

}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", FileServer::from("/app/static"))
    .mount("/query", routes!(raw_query))
    .mount("/api", routes!(index))
}
