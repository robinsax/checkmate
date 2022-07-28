use serde::{Deserialize, Serialize};

use rocket::{Route, options, post, get, routes};
use rocket::serde::json::Json;
use rocket::http::Status;

use super::super::state::{User, AuthToken, AuthRegistryState};
use super::super::responses::{ResponseCase, error_response};

#[derive(Serialize, Clone)]
pub struct AuthTokenInfo {
    pub token: String
}

#[derive(Deserialize, Clone)]
pub struct AuthConstructor {
    pub name: String
}

#[options("/")]
fn auth_endpoint_cors() -> Status {
    Status::Ok
}

#[get("/")]
async fn auth_endpoint_get(
    token: AuthToken, state: &AuthRegistryState
) -> ResponseCase<User> {
    let mut auth = state.lock().await;

    match auth.user_for(&token) {
        Ok(user) => ResponseCase::Success(user),
        Err(err) => error_response(401, err)
    }
}

#[post("/", format="application/json", data="<body>")]
async fn auth_endpoint_post(
    body: Json<AuthConstructor>, state: &AuthRegistryState
) -> ResponseCase<AuthTokenInfo> {
    let mut auth = state.lock().await;

    match auth.add_user(&body.name) {
        Ok(token) => ResponseCase::Success(AuthTokenInfo{token: token}),
        Err(err) => error_response(422, err)
    }
}

pub fn routes() -> Vec<Route> {
    routes![auth_endpoint_cors, auth_endpoint_get, auth_endpoint_post]
}
