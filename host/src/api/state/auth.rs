use std::sync::Arc;
use std::collections::HashMap;

use log::info;
use tokio::sync::Mutex;
use uuid::Uuid;
use regex::Regex;
use lazy_static::lazy_static;

use serde::Serialize;

use rocket::{State, async_trait};
use rocket::request::{Outcome, Request, FromRequest};

use super::super::responses::error_outcome;

pub type AuthRegistryState = State<Arc<Mutex<AuthRegistry>>>;

#[derive(Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String
}

pub struct AuthRegistry {
    tokens: HashMap<String, User>
}

pub struct AuthToken(String);

fn parse_authz_header(text: &str) -> Option<String> {
    lazy_static! {
        static ref BEARER_HEADER_RE: Regex = Regex::new("^Bearer ([A-Za-z0-9\\-]+)$").unwrap();
    }

    let capture = match BEARER_HEADER_RE.captures(text) {
        Some(m) => m,
        _ => return None,
    };

    match capture.get(1) {
        Some(t) => Some(t.as_str().to_string()),
        _ => None
    }
}

impl AuthRegistry {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new()
        }
    }

    pub fn into_state(self) -> Arc<Mutex<AuthRegistry>> {
        Arc::new(Mutex::new(self))
    }

    pub fn add_user(&mut self, name: &String) -> Result<String, &'static str> {
        let token = Uuid::new_v4().to_string();
        let id = Uuid::new_v4().to_string();

        info!("auth_token: {} to {}", token, name);

        self.tokens.insert(token.clone(), User{id, name: name.clone()});

        Ok(token)
    }

    pub fn user_for(&mut self, token: &AuthToken) -> Result<User, &'static str> {
        match self.tokens.get(token.0.as_str()) {
            Some(user) => Ok(user.clone()),
            _ => Err("invalid token")
        }
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let value = match req.headers().get_one("Authorization") {
          Some(v) => v,
          _ => return error_outcome(401, "no authz header"),
        };

        let token = match parse_authz_header(value) {
            Some(v) => v,
            _ => return error_outcome(401, "invalid token format"),
        };

        Outcome::Success(Self(token))
    }
}
