
use std::time::{SystemTime, UNIX_EPOCH};

use rocket::form::{FromForm, Form};
use rocket::http::{Status, CookieJar, Cookie};
use rocket::serde::{Serialize, Deserialize};

use crate::db;
use super::tokens::ToJWT;
use super::{UserRole, UserSession};

#[derive(FromForm)]
pub struct MfaForm {
    auth_token: String,

    #[field(validate = len(6..=6))] //the code's length must be between 6 and 6, inclusive
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MfaToken {
    pub iat: u64,
    pub exp: u64,
    pub login_id: u64,
    pub user: String,
}

impl MfaToken {

    pub fn new(login_id: u64, user: String) -> Self {

        let iat = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap().as_secs();
        let exp = iat + 3600;

        Self {
            iat,
            exp,
            login_id,
            user
        }
    }
}

impl ToJWT for MfaToken {}

#[post("/mfa", data = "<req_form>")]
pub async fn mfa_auth(req_form: Form<MfaForm>, db_pool: &db::Users, cookies: &CookieJar<'_>) -> Result<Status, Status> {

    // Grab everything from the form
    let MfaForm {auth_token, code} = req_form.into_inner();

    // Grab everything from the JWT
    let MfaToken {login_id, user, ..} =
    MfaToken::from_jwt(auth_token)
    .ok_or(Status::BadRequest)?
    .claims;

    // Check if the user attempted to log in in the last minute.
    // If so, get the id of that log (so we can update it)
    // and the user's mfa secret
    let (user_secret, user_name, user_role): (Vec<u8>, String, UserRole) = sqlx::query_as(
        "SELECT users.mfa_secret, users.username, CAST(users.role AS UNSIGNED)-1
        FROM access_log
        INNER JOIN users ON access_log.user_found = users.username
        WHERE access_log.user_found = ?
        AND access_log.id = ?
        AND access_log.timestamp >= NOW() - INTERVAL 1 MINUTE"
    )
    .bind(&user)
    .bind(&login_id)
    .fetch_optional(&mut db_pool.acquire().await.or(Err(Status::ServiceUnavailable))?)
    .await
    .or_else(|e| {
        eprintln!("   !! Got err getting log from db: {e}");
        Err(Status::InternalServerError)
    })?
    .ok_or(Status::BadRequest)?;            // Then handle the case where no records exist

    // Actually check the MFA secret
    let correct = code == mfalib::gen_mfa(user_secret[..].try_into().or(Err(Status::BadRequest))?);

    let rv = if correct {

        let user_session = UserSession::new(user_name.clone(), user_role);

        sqlx::query("UPDATE users SET session = ? WHERE username = ?")
        .bind(&user_session.session)
        .bind(&user_name)
        .execute(&mut db_pool.acquire().await.or(Err(Status::ServiceUnavailable))?)
        .await
        .or_else(|e| {
            eprintln!("   !! Got err setting session cookie: {e}");
            Err(Status::InternalServerError)
        })?;

        let token = user_session.to_jwt()
        .or_else(|e| {
            eprintln!("   !! Got err generating JWT: {e}");
            Err(Status::InternalServerError)
        })?;
        
        cookies.add(Cookie::new("session_token", token));

        Ok(Status::Ok)
    }
    else {
        Ok(Status::Unauthorized)
    };

    sqlx::query("UPDATE access_log SET mfa_success = ? WHERE id = ?")
    .bind(&correct)
    .bind(login_id)
    .execute(&mut db_pool.acquire().await.or(Err(Status::ServiceUnavailable))?)
    .await
    .or_else(|e| {
        eprintln!("   !! Got err updating access logs: {e}");
        Err(Status::InternalServerError)
    })?;

    rv

}