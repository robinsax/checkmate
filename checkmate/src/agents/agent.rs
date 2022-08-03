use async_trait::async_trait;

use crate::model::{State, Move};

#[async_trait]
pub trait Agent: Send {
    async fn get_move_for_model(&mut self, state: &State) -> Move;
}
