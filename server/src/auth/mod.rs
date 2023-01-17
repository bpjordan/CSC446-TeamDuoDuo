use argon2::{Argon2, PasswordVerifier, PasswordHash};
use rand::RngCore;
use rocket::futures::TryFutureExt;
use rocket::http::{CookieJar, Status, Cookie};
use rocket::form::Form;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::sqlx;

mod tokens;
use crate::db;

pub use tokens::UserSession;

#[derive(Debug, FromForm)]
pub struct UserCredentials {
    username: String,
    password: String,
}

#[post("/login", data="<creds>")]
pub(crate) async fn login_handler(creds: Form<UserCredentials>, cookies: &CookieJar<'_>, db_pool: &db::Users) -> Result<Status, Status> {

    let query_connection = &mut db_pool.acquire().await.map_err(|_| Status::ServiceUnavailable)?;
    sqlx::query("SELECT username, password FROM users WHERE username = ?")
    .bind(&creds.username)
    .fetch_optional(query_connection)   // returns a future, so all handling needs to be instructions
                                // for what to do when the future finishes

    // Keep errors as one type so we can handle them at the end
    .or_else(|_| async move {Err(Status::InternalServerError)})

    // Assuming the database didn't error, we can check that the username exists
    // and the password is correct
    .and_then(|res| async {

        // Returns Some only if username exists and passwords match
        let valid_user = res.and_then(|row| {
            let username: String = row.try_get(0).ok()?;
            let password: String = row.try_get(1).ok()?;

            Argon2::default().verify_password(
                creds.password.as_bytes(),
                &PasswordHash::new(&password).ok()?
            ).ok()
            .and(Some(username))
        });

        if let Some(valid_username) = &valid_user {
            let mut session = [0u8; 16];
            rand::thread_rng().fill_bytes(&mut session);
            let session = hex::encode_upper(session);

            let token = tokens::generate_jwt(valid_username.clone(), session.clone());

            cookies.add_private(Cookie::new("session_token", token));

            sqlx::query("UPDATE users SET session = ? WHERE username = ?")
            .bind(&session)
            .bind(&valid_username)
            .execute(&mut db_pool.acquire().await.map_err(|_| Status::ServiceUnavailable)?).await
            .map_err(|_| Status::InternalServerError)?;

        };

        Ok(valid_user)
    })
    .and_then(|valid_user| async move{
        match valid_user {
            Some(_) => Ok(Status::Ok),
            None => Ok(Status::Unauthorized),
        }
    })
    .await
}