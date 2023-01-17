
use std::str::FromStr;

use rocket::outcome::IntoOutcome;
use rocket::request::Outcome;
use rocket::{request::FromRequest, Request};
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::Connection;

use crate::db;

use super::tokens;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub enum UserRole {
    Trainer,
    Professor,
    GymLeader,
}

impl FromStr for UserRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "trainer" => Ok(Self::Trainer),
            "professor" => Ok(Self::Professor),
            "leader" => Ok(Self::GymLeader),
            _ => Err(())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserSession {
    pub iat: u64,
    pub exp: u64,
    pub user: String,
    pub session: String,
    pub role: UserRole,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let conn = req.guard::<Connection<db::Users>>().await.unwrap();

        let claims = req.cookies()
            .get("session_token")
            .and_then(|c| tokens::parse_jwt(c.value()))
            .and_then(|t| Some(t.claims));

        let valid = match &claims {
            Some(c) => tokens::is_valid(c, conn).await,
            None => false,
        };

        claims
            .filter(|_| valid)
            .or_forward(())

    }
}