#[macro_use] extern crate rocket;

use rocket::{Rocket, Build};

mod auth;
mod query;
mod db;

mod frontend {
    use rocket::response::Redirect;
    use rocket::fs::FileServer;
    use rocket::fairing::AdHoc;


    #[get("/")]
    fn redirect_to_login() -> Redirect {
        Redirect::to(uri!("/login.html"))
    }

    pub fn stage() -> AdHoc {
        AdHoc::on_ignite("Frontend", |rocket| async {
            rocket
            .mount("/", FileServer::from("/app/static"))
            .mount("/", routes!(redirect_to_login))
        })
    }

}

#[launch]
fn rocket() -> Rocket<Build> {

    rocket::build()
    .attach(db::stage())
    .attach(auth::stage())
    .attach(query::stage())
    .attach(frontend::stage())

}
