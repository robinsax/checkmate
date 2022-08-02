use super::super::model::{State, Move};
use super::super::errors::ValidationError;

pub trait StateFormat {
    fn parse_state(&self) -> Result<State, ValidationError>;
}

pub trait MoveFormat {
    fn parse_move(&self, state: &State) -> Result<Move, ValidationError>;
}
