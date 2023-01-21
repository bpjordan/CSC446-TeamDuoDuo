
use rand::RngCore;
use jsonwebtoken::{TokenData, Header, Validation, EncodingKey, DecodingKey};
use rocket::serde::DeserializeOwned;
use rocket_db_pools::{Connection, sqlx};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db;
use super::roles::{UserSession, UserRole};

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


pub fn parse_jwt<'a, T: DeserializeOwned> (token: &'a str) -> Option<TokenData<T>> {
    jsonwebtoken::decode(
        &token,
        &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
        &Validation::default()
    ).ok()
}

pub(crate) async fn is_valid(claim: &UserSession ,mut conn: Connection<db::Users>) -> bool {

    sqlx::query(
        "SELECT username FROM users WHERE username = ? AND session = ?"
    ).bind(&claim.user).bind(&claim.session)
    .fetch_optional(&mut *conn).await.unwrap()
    .is_some()

}

pub fn generate_jwt(user: String, role: UserRole, session: String) -> String {
    let now = SystemTime::now()
    .duration_since(UNIX_EPOCH).unwrap().as_secs();
    let expires = now + 3600;   //Sessions expire after 1 hour

    let session_token = UserSession {
        iat: now,
        exp: expires,
        user,
        session,
        role,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &session_token,
        &EncodingKey::from_secret(get_jwt_secret().as_bytes())
    ).unwrap()
}