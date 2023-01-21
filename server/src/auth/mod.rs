use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordVerifier, PasswordHash, PasswordHasher};
use rand::RngCore;
use rand::rngs::OsRng;
use rocket::fairing::AdHoc;
use rocket::futures::{TryFutureExt, FutureExt};
use rocket::http::{CookieJar, Status, Cookie};
use rocket::form::Form;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::sqlx;

mod tokens;
mod roles;
mod mfa;
use crate::db;

pub use roles::{UserSession, UserRole, TrainerSession, ProfessorSession, GymLeaderSession};

#[derive(Debug, FromForm)]
pub struct UserCredentials {
    username: String,
    password: String,
}

#[post("/login", data="<req_creds>")]
async fn login_handler(req_creds: Form<UserCredentials>, cookies: &CookieJar<'_>, db_pool: &db::Users) -> Status {

    async {
        let query_connection = &mut db_pool.acquire().await?;
        sqlx::query("SELECT username, password, role FROM users WHERE username = ?")
        .bind(&req_creds.username)
        .fetch_optional(query_connection).await   // returns a future, so all handling needs to be instructions
                                            // for what to do when the future finishes
    }

    // Assuming the database didn't error, we can check that the username exists
    // and the password is correct
    .and_then(|res| async {

        // Returns Some only if username exists and passwords match
        let valid_user = res.and_then(|row| {
            let username = row.try_get::<String, _>(0).ok()?;
            let password = row.try_get::<String, _>(1).ok()?;
            let role = row.try_get::<String, _>(2).ok()?.parse::<UserRole>().ok()?;

            Argon2::default().verify_password(
                req_creds.password.as_bytes(),
                &PasswordHash::new(&password).ok()?
            ).ok()
            .and(Some((username, role)))
        });

        // Create or remove the session token
        match &valid_user {
            Some((valid_username, role)) => {
                let mut session = [0u8; 16];
                rand::thread_rng().fill_bytes(&mut session);
                let session = hex::encode_upper(session);

                let token = tokens::generate_jwt(valid_username.clone(), role.clone(), session.clone());

                cookies.add(Cookie::new("session_token", token));

                sqlx::query("UPDATE users SET session = ? WHERE username = ?")
                .bind(&session)
                .bind(&valid_username)
                .execute(&mut db_pool.acquire().await?).await?;

            }
            None => cookies.remove(Cookie::named("session_token")),
        };

        Ok(valid_user)
    })
    

    // Log result
    .then(|result| async {

        let salt = SaltString::generate(&mut OsRng);
        let pw_hash = Argon2::default().hash_password(&req_creds.password.as_bytes(), &salt)
            .map_or("error hashing password".into(), |h| h.to_string());

        let (success, user, error, status) = match &result{
            Ok(Some((user, _))) => (true, Some(user), None, Status::Ok),
            Ok(None) => (false, None, None, Status::Unauthorized),
            Err(e) => (false, None, Some(e.to_string()), Status::InternalServerError)
        };

        sqlx::query("INSERT INTO access_log(username_provided, password_provided, success, user_found, session_len, error) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(&req_creds.username)
        .bind(pw_hash)
        .bind(success)
        .bind(user)
        .bind(3600)
        .bind(error)
        .execute(&mut db_pool.acquire().await?).await?;

        result.and(Ok(status))
    })

    // Handle errors by printing to stdout and returning a 500 code
    .unwrap_or_else(|e| {
        println!("   !! {e}");
        Status::InternalServerError
    })

    .await
}

// Function called by main to add module to the api
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth endpoints", |rocket| async {
        rocket.mount("/api", routes![login_handler, mfa::mfa_auth])
    })
}