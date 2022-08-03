use rand::thread_rng;
use rand::seq::SliceRandom;

use async_trait::async_trait;

use crate::model::{State, Move};
use super::agent::Agent;

#[derive(Clone)]
pub struct HeurRandAgent;

#[async_trait]
impl Agent for HeurRandAgent {
    async fn get_move_for_model(&mut self, state: &State) -> Move {
        let moves = &state.get_legal_moves()[..];
        let mut rng = thread_rng();

        moves.choose(&mut rng).unwrap().clone()
    }
}

impl HeurRandAgent {
    pub fn new() -> Self {
        Self{}
    }
}
