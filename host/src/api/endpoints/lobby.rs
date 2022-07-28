use uuid::Uuid;

use rocket::{Route, options, post, routes};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::http::Status;

use super::super::state::{AuthToken, GameHostState, AuthRegistryState};
use super::super::responses::{ResponseCase, error_response};

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")] 
pub enum GameJoinAction {
    New
}

#[derive(Deserialize)]
pub struct GameJoinRequest {
    game: GameJoinAction
}

#[derive(Serialize, Clone)]
pub struct GameJoinResponse {
    pub id: String
}

#[options("/")]
fn lobby_endpoint_cors() -> Status {
    Status::Ok
}

#[post("/", format="application/json", data="<body>")]
async fn lobby_endpoint_post(
    token: AuthToken, body: Json<GameJoinRequest>,
    games_state: &GameHostState, auth_state: &AuthRegistryState
) -> ResponseCase<GameJoinResponse> {
    let mut games = games_state.lock().await;

    match body.game {
        GameJoinAction::New => {
            let id = Uuid::new_v4().to_string();
            games.create_game(&id).await;

            ResponseCase::Success(GameJoinResponse{id})
        }
    }
}

pub fn routes() -> Vec<Route> {
    routes![lobby_endpoint_cors, lobby_endpoint_post]
}
