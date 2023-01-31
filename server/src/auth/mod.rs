use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordVerifier, PasswordHash, PasswordHasher};
use rand::rngs::OsRng;
use rocket::fairing::AdHoc;
use rocket::futures::{TryFutureExt, FutureExt};
use rocket::http::Status;
use rocket::form::Form;
use rocket::serde::json::Value;
use rocket::serde::json::serde_json::json;
use rocket_db_pools::sqlx;

mod tokens;
mod roles;
mod mfa;
use crate::db;

pub use roles::{UserSession, UserRole, TrainerSession, ProfessorSession, GymLeaderSession};

use self::mfa::MfaToken;
use self::tokens::ToJWT;

#[derive(Debug, FromForm)]
pub struct UserCredentials {
    username: String,
    password: String,
}

#[post("/login", data="<req_creds>")]
async fn login_handler(req_creds: Form<UserCredentials>, db_pool: &db::Users) -> Result<Value, Status> {

    let UserCredentials { username: req_username, password: req_password } = req_creds.into_inner();

    let query_username = req_username.clone();
    let check_password = req_password.clone();

    async {
        let query_connection = &mut db_pool.acquire().await?;
        sqlx::query_as::<_, (String, String, UserRole)>("SELECT username, password, CAST(role AS UNSIGNED)-1 FROM users WHERE username = ?")
        .bind(query_username)
        .fetch_optional(query_connection).await
    }

    // Assuming the database didn't error, we can check that the username exists
    // and the password is correct
    .and_then(|res| async {

        // Returns Some only if username exists and passwords match
        Ok(res.and_then(|(username, password, role)| {
            Argon2::default().verify_password(
                check_password.as_bytes(),
                &PasswordHash::new(&password).ok()?
            ).ok()
            .and(Some((username, role)))
        }))
    })
    

    // Log result
    .then(|result| async move {

        let salt = SaltString::generate(&mut OsRng);
        let pw_hash = Argon2::default().hash_password(req_password.as_bytes(), &salt)
            .map_or("error hashing password".into(), |h| h.to_string());

        let (success, user, error) = match result{
            Ok(Some((user, _))) => (true, Some(user), None),
            Ok(None) => (false, None, None),
            Err(e) => (false, None, Some(e.to_string()))
        };

        let log_id = sqlx::query(
            "INSERT INTO access_log(
                username_provided,
                password_provided,
                success,
                user_found,
                session_len,
                error
            )
            VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&req_username)
        .bind(pw_hash)
        .bind(success)
        .bind(&user)
        .bind(3600)
        .bind(error)
        .execute(&mut db_pool.acquire().await.or(Err(Status::ServiceUnavailable))?)
        .await
        .or_else(|e| {
            eprintln!("   !! Couldn't insert log: {e}");
            Err(Status::InternalServerError)
        })?
        .last_insert_id();

        if success {

            let token = user.ok_or(Status::InternalServerError)
            .and_then(|u|
                MfaToken::new(log_id, u).to_jwt().or(Err(Status::InternalServerError))
            )?;

            Ok(json!({"mfa_token": token}))

        } else {
            Err(Status::Unauthorized)
        }
    })

    // Handle errors by printing to stdout and returning a 500 code
    .await
}

// Function called by main to add module to the api
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth endpoints", |rocket| async {
        rocket.mount("/api", routes![login_handler, mfa::mfa_auth])
    })
}