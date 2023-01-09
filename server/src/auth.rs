
use rand::RngCore;
use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::Status;
use rocket::serde::{Serialize, Deserialize};
use jsonwebtoken::{TokenData, Header, Validation, EncodingKey, DecodingKey};
use rocket_db_pools::{Connection, sqlx};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserSession {
    pub iat: u64,
    pub exp: u64,
    pub user: String,
    pub session: String,
}

#[derive(Debug)]
pub enum AuthError {
    _MissingKey,
    InvalidKey
}

const SECRET_ENV_VAR: &'static str = "SERVER_JWT_SECRET";

pub fn get_jwt_secret() -> String {
    match std::env::var(SECRET_ENV_VAR) {
        Ok(s) => s,
        Err(_) => {
            let mut secret = [0u8; 64];
            rand::thread_rng().fill_bytes(&mut secret);
            let secret = hex::encode_upper(secret);
            std::env::set_var(SECRET_ENV_VAR, &secret);
            secret
        },
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let conn = req.guard::<Connection<db::Users>>().await.unwrap();
        if let Some(t) = req.headers().get_one("Authorization") {
            let claim = match parse_jwt(t) {
                Ok(t) => t.claims,
                Err(_) => return Outcome::Failure((Status::BadRequest, AuthError::InvalidKey)),
            };
            if is_valid(&claim, conn).await {
                return Outcome::Success(claim)
            }
        }

        Outcome::Forward(())

    }
}

pub fn parse_jwt<'a> (token: &'a str) -> jsonwebtoken::errors::Result<TokenData<UserSession>> {
    jsonwebtoken::decode(
        &token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::default()
    )
}

async fn is_valid(claim: &UserSession ,mut conn: Connection<db::Users>) -> bool {

    sqlx::query(
        "SELECT username FROM users WHERE username = ? AND session = ?"
    ).bind(&claim.user).bind(&claim.session)
    .fetch_optional(&mut *conn).await.unwrap()
    .is_some()

}

pub fn generate_jwt(user: String, session: String) -> String {
    let now = SystemTime::now()
    .duration_since(UNIX_EPOCH).unwrap().as_secs();
    let expires = now + 3600;   //Sessions expire after 1 hour

    let session_token = UserSession {
        iat: now,
        exp: expires,
        user,
        session,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &session_token,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes())
    ).unwrap()
}