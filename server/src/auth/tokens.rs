
use rand::RngCore;
use jsonwebtoken::{TokenData, Header, Validation, EncodingKey, DecodingKey};
use rocket::serde::{DeserializeOwned, Serialize};
use rocket_db_pools::{Connection, sqlx};

use crate::db;
use super::roles::UserSession;

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

pub(crate) async fn is_valid(claim: &UserSession ,mut conn: Connection<db::Users>) -> bool {

    sqlx::query(
        "SELECT username FROM users WHERE username = ? AND session = ?"
    ).bind(&claim.user).bind(&claim.session)
    .fetch_optional(&mut *conn).await.unwrap()
    .is_some()

}

pub trait ToJWT 
    where Self: Serialize + DeserializeOwned
    {
    fn to_jwt(&self) -> Result<String, &'static str> {
        jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(get_jwt_secret().as_bytes())
        )
        .or(Err("Couldn't generate JWT"))
    }

    fn from_jwt(token: String) -> Option<TokenData<Self>> {
        jsonwebtoken::decode(
            &token,
            &DecodingKey::from_secret(get_jwt_secret().as_bytes()),
            &Validation::default()
        ).ok()
    }
}
