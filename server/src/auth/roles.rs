
use std::ops::Deref;
use std::str::FromStr;

use rocket::http::Status;
use rocket::outcome::{IntoOutcome};
use rocket::request::Outcome;
use rocket::{request::FromRequest, Request};
use rocket::serde::{Serialize, Deserialize};
use rocket_db_pools::Connection;
use sqlx::Type;

use crate::db;

use super::tokens;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Type, PartialEq, Eq, PartialOrd, Ord)]
#[serde(crate = "rocket::serde")]
#[repr(u64)] // We have to do some weird hacky garbage to make sqlx cooperate with this being an enum
pub enum UserRole {
    Trainer,
    Professor,
    GymLeader,
}

impl FromStr for UserRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trainer" => Ok(Self::Trainer),
            "professor" => Ok(Self::Professor),
            "leader" | "gymleader" | "gym_leader" => Ok(Self::GymLeader),
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

// Instructions for how to get the user's session data from a request
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let conn = req.guard::<Connection<db::Users>>().await;

        let conn = match conn {
            Outcome::Success(c) => c,
            _ => return Outcome::Failure((Status::InternalServerError, ()))
        };

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

// Simple wrapper structs on a user session that first checks if the user has the appropriate permissions
pub struct TrainerSession(UserSession);
pub struct ProfessorSession(UserSession);
pub struct GymLeaderSession(UserSession);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TrainerSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.guard::<UserSession>().await {
            Outcome::Success(s) if s.role >= UserRole::Trainer => Outcome::Success(Self(s)),
            Outcome::Success(_) => Outcome::Failure((Status::Forbidden, ())),
            Outcome::Forward(e) => Outcome::Forward(e),
            Outcome::Failure(e) => Outcome::Failure(e),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ProfessorSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.guard::<UserSession>().await {
            Outcome::Success(s) if s.role >= UserRole::Professor => Outcome::Success(Self(s)),
            Outcome::Success(_) => Outcome::Failure((Status::Forbidden, ())),
            Outcome::Forward(e) => Outcome::Forward(e),
            Outcome::Failure(e) => Outcome::Failure(e),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GymLeaderSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.guard::<UserSession>().await {
            Outcome::Success(s) if s.role >= UserRole::GymLeader => Outcome::Success(Self(s)),
            Outcome::Success(_) => Outcome::Failure((Status::Forbidden, ())),
            Outcome::Forward(e) => Outcome::Forward(e),
            Outcome::Failure(e) => Outcome::Failure(e),
        }
    }
}

// Make the various session types transparent wrappers around their associated UserSession
impl Deref for TrainerSession {
    type Target = UserSession;

    fn deref(&self) -> &Self::Target { &self.0 }
}


impl Deref for ProfessorSession {
    type Target = UserSession;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl Deref for GymLeaderSession {
    type Target = UserSession;

    fn deref(&self) -> &Self::Target { &self.0 }
}