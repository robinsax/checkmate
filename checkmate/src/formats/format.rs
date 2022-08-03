use crate::model::{State, Move, Piece, Position, PieceType};
use crate::errors::ValidationError;

pub trait ToPosition {
    fn to_position(self) -> Result<Position, ValidationError>;
}

pub trait ToPieceType {
    fn to_piece_type(self) -> Result<PieceType, ValidationError>;
}

pub trait ToPiece {
    fn to_piece(self) -> Result<Piece, ValidationError>;
}

pub trait ToMove {
    fn to_move(self, state: &State) -> Result<Move, ValidationError>;
}

pub trait ToState {
    fn to_state(self) -> Result<State, ValidationError>;
}
