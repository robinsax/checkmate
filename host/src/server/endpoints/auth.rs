use std::sync::{ Arc, Mutex };

use uuid::Uuid;
use regex::Regex;
use log::{ info };
use bimap::{ BiMap, BiHashMap };
use lazy_static::lazy_static;

use rocket::{ State, Route, options, post, get, routes, async_trait };
use rocket::serde::{ Deserialize, Serialize };
use rocket::serde::json::{ Json };
use rocket::http::{ Status };
use rocket::request::{ Outcome, Request, FromRequest };

use super::super::responses::{ ResponseCase, error_response, error_outcome };

type SafeAuthRegistry = Arc<Mutex<AuthRegistry>>;

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthInfo {
    pub name: String
}

#[derive(Serialize, Clone)]
pub struct AuthTokenInfo {
    pub token: String
}

pub struct AuthRegistry {
    tokens: BiMap<String, String>
}

pub struct AuthToken(pub String);

fn parse_authz_header(text: &str) -> Option<String> {
    lazy_static! {
        static ref BEARER_HEADER_RE: Regex = Regex::new("^Bearer ([A-Za-z\\-]+)$")
            .expect("authz header parser invalid");
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
            tokens: BiHashMap::new()
        }
    }

    pub fn add_user(&mut self, name: String) -> Result<String, &'static str> {
        if self.tokens.contains_right(name.as_str()) {
            return Err("name taken");
        }

        let token = Uuid::new_v4().to_string();

        info!("issued token {} to {}", token, name);

        self.tokens.insert(token.clone(), name);

        Ok(token)
    }

    pub fn validate(&mut self, token: String) -> Result<String, &'static str> {
        match self.tokens.get_by_left(token.as_str()) {
            Some(name) => Ok(name.to_string()),
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

#[options("/")]
fn auth_endpoint_cors() -> Status {
    Status::Ok
}

#[get("/")]
fn auth_endpoint_get(token: AuthToken, state: &State<SafeAuthRegistry>) -> ResponseCase<AuthInfo> {
    let mut registry = state.lock().expect("auth system tainted");
    
    let name = match registry.validate(token.0) {
        Ok(r) => r,
        Err(err) => return error_response(422, err),
    };

    ResponseCase::Success(AuthInfo{name: name})
}

#[post("/", format="application/json", data="<body>")]
fn auth_endpoint_post(body: Json<AuthInfo>, state: &State<SafeAuthRegistry>) -> ResponseCase<AuthTokenInfo> {
    let mut registry = state.lock().expect("auth system tained");

    let token = match registry.add_user(body.name.clone()) {
        Ok(r) => r,
        Err(err) => return error_response(422, err),
    };

    ResponseCase::Success(AuthTokenInfo{token: token})
}

pub fn auth_routes() -> Vec<Route> {
    routes![auth_endpoint_cors, auth_endpoint_get, auth_endpoint_post]
}
