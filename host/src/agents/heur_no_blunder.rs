use rand::thread_rng;
use rand::seq::SliceRandom;

use async_trait::async_trait;

use crate::chess::model::{State, Move};
use crate::chess::agent::ModelAgent;

#[derive(Clone)]
pub struct HeurNoBlunderAgent;

#[async_trait]
impl ModelAgent for HeurNoBlunderAgent {
    async fn get_move_for_model(&mut self, state: &State) -> Move {
        let moves = &state.legal_moves()[..];
        let mut rng = thread_rng();

        moves.choose(&mut rng).unwrap().clone()
    }
}

impl HeurNoBlunderAgent {
    pub fn new() -> Self {
        Self{}
    }
}
