use serde::Deserialize;

use rocket::{Route, options, put, get, routes};
use rocket::serde::json::Json;
use rocket::http::Status;

use crate::chess::format::StateFormat;

use super::super::state::{AuthToken, AuthRegistryState, GameHostState};
use super::super::responses::{ResponseCase, error_response};

#[derive(Deserialize)]
pub struct TurnAction {
    r#move: String
}

#[options("/<id>")]
fn game_endpoint_cors(id: String) -> Status {
    Status::Ok
}

#[get("/<id>")]
async fn game_endpoint_get(
    id: String, games_state: &GameHostState
) -> ResponseCase<StateFormat> {
    let games = games_state.lock().await;

    ResponseCase::Success(games.get_state(&id).await)
}

#[put("/<id>", format="application/json", data="<body>")]
async fn game_endpoint_put(
    token: AuthToken, id: String, body: Json<TurnAction>,
    games_state: &GameHostState, auth_state: &AuthRegistryState
) -> ResponseCase<StateFormat> {
    let games = games_state.lock().await;

    let state = match games.make_move(&id, &body.r#move).await {
        Ok(s) => s,
        Err(err) => return error_response(422, err),
    };

    ResponseCase::Success(state)
}

pub fn routes() -> Vec<Route> {
    routes![game_endpoint_cors, game_endpoint_get, game_endpoint_put]
}
