
use rand::RngCore;
use rocket::form::{FromForm, Form};
use rocket::http::{Status, CookieJar, Cookie};
use rocket::serde::{Serialize, Deserialize};

use crate::db;
use super::{tokens, UserRole};

#[derive(FromForm)]
pub struct MfaForm {
    auth_token: String,

    #[field(validate = len(..=6))]
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MfaToken {
    pub iat: u64,
    pub exp: u64,
    pub user: String,
}

#[post("/mfa", data = "<req_form>")]
pub async fn mfa_auth(req_form: Form<MfaForm>, db_pool: &db::Users, cookies: &CookieJar<'_>) -> Result<Status, Status> {

    let user: MfaToken = tokens::parse_jwt(&req_form.auth_token)
    .ok_or(Status::BadRequest)?
    .claims;

    // Check if the user attempted to log in in the last minute.
    // If so, get the id of that log (so we can update it)
    // and the user's mfa secret
    let (log_id, user_secret, user_name, user_role): (u64, Vec<u8>, String, UserRole) = sqlx::query_as(
        "SELECT access_log.id, users.mfa_secret, users.name, users.role
        FROM access_log
        INNER JOIN users ON access_log.user_found = users.username
        WHERE access_log.user_found = ?
        AND access_log.timestamp >= NOW() - INTERVAL 1 MINUTE"
    )
    .bind(&user.user)
    .fetch_optional(&mut db_pool.acquire().await.or(Err(Status::ServiceUnavailable))?)
    .await
    .or(Err(Status::InternalServerError))?  // First handle any DB errors
    .ok_or(Status::BadRequest)?;            // Then handle the case where no records exist

    // Actually check the MFA secret
    let correct = req_form.code == mfalib::gen_mfa(user_secret[..].try_into().or(Err(Status::BadRequest))?);

    let rv = if correct {
        let mut session = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut session);
        let session = hex::encode_upper(session);

        let token = tokens::generate_jwt(user_name.clone(), user_role, session.clone());

        cookies.add(Cookie::new("session_token", token));

        sqlx::query("UPDATE users SET session = ? WHERE username = ?")
        .bind(&session)
        .bind(&user_name)
        .execute(&mut db_pool.acquire().await.or(Err(Status::ServiceUnavailable))?)
        .await.or(Err(Status::InternalServerError))?;

        Ok(Status::Ok)
    }
    else {
        Ok(Status::Unauthorized)
    };

    sqlx::query("UPDATE access_log SET mfa_success = ? WHERE id = ?")
    .bind(&correct)
    .bind(log_id)
    .execute(&mut db_pool.acquire().await.or(Err(Status::ServiceUnavailable))?)
    .await
    .or(Err(Status::InternalServerError))?;

    rv

}