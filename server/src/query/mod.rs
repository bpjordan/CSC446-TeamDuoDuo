
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::auth;

mod users;
pub(crate) mod log;
mod pokemon;

// Endpoint that lists allowed queries for a user
#[get("/")]
async fn query_list(user_session: auth::UserSession) -> Json<Vec<&'static str>> {
    let allowed_endpoints = match user_session.role {
        auth::UserRole::Trainer => vec!["user, user_pokemon"],
        auth::UserRole::Professor => vec!["user, user_pokemon, pokemon"],
        auth::UserRole::GymLeader => vec!["user", "user_pokemon", "pokemon",
                                        "users", "access_log"],
    };

    Json(allowed_endpoints)
}

// Fallback endpoint for unauthenticated calls to `query_list`
#[get("/", rank = 2)]
fn list_unauthenticated() -> Status {
    Status::Unauthorized
}

// Fallback endpoint that responds to any queries that aren't allowed for an authenticated user
#[get("/<_endpoint>", rank = 2)]
fn _query_forbidden(_endpoint: String, _user_session: auth::UserSession) -> Status {
    Status::Forbidden
}

// Fallback endpoint that responds to any queries that aren't allowed for an unauthenticated user
#[get("/<_endpoint>", rank = 3)]
fn query_unauthorized(_endpoint: String) -> Status {
    Status::Unauthorized
}

// Function called by main to add module to the api
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Query endpoints", |rocket| async {
        rocket
        .mount("/api/query", routes![
            query_list, list_unauthenticated,
            users::query_users, users::query_current_user,
            pokemon::query_pokemon, pokemon::query_user_pokemon,
            log::query_logs,
            query_unauthorized
        ])
    })
}
