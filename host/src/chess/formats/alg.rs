use std::fmt;

use regex::Regex;
use lazy_static::lazy_static;

use super::super::model::{State, Move, PieceType, Position, Color};
use super::super::errors::ValidationError;
use super::format::{StateFormat, MoveFormat};

pub trait ToAlg {
    fn to_alg(&self) -> AlgNotation;
}

pub struct AlgNotation(String);

impl fmt::Display for AlgNotation {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{}", self.0)
    }
}

impl MoveFormat for AlgNotation {
    fn parse_move(&self, state: &State) -> Result<Move, ValidationError> {
        lazy_static! {
            static ref MOVE_RE: Regex = Regex::new("^([A-Z]|[a-h]?)(x?)([a-h][0-8])$").unwrap();
        }

        let move_str = &self.0;
        let moves = state.get_legal_moves();

        let king_side = move_str == "O-O";
        if king_side || move_str == "O-O-O" {
            return Err(ValidationError::InvalidState{token: move_str.to_owned()});
        }

        let captures = match MOVE_RE.captures(move_str) {
            Some(m) => m,
            None => return Err(ValidationError::InvalidState{token: move_str.to_owned()}),
        };

        let type_str = captures.get(1).unwrap().as_str();
        let capture_str = captures.get(2).unwrap().as_str();
        let dest_str = captures.get(3).unwrap().as_str();

        let dest_position = Position::from_alg(dest_str)?;

        let is_capture = capture_str.len() > 0;
        if is_capture && state.board[&dest_position].as_ref().is_none() {
            return Err(ValidationError::InvalidState{token: dest_str.to_owned()})
        }

        let piece_type = self.piece_type_for(type_str)?;
        let pawn_file = match piece_type {
            PieceType::Pawn => Some(type_str),
            _ => None
        };

        for check_move in moves {
            if check_move.to != dest_position {
                continue;
            }
            if check_move.piece.piece_type != piece_type {
                continue;
            }
            if piece_type == PieceType::Pawn && is_capture && !check_move.from.is_file(pawn_file.unwrap()) {
                continue;
            }

            return Ok(check_move.clone());
        }

        Err(ValidationError::InvalidState{token: move_str.to_owned()})
    }
}

impl AlgNotation {
    pub fn new(data: impl AsRef<str>) -> Self {
        Self(data.as_ref().to_string())
    }

    fn piece_type_for(&self, repr: impl AsRef<str>) -> Result<PieceType, ValidationError> {
        let repr_str = repr.as_ref();
        if repr_str.len() == 0 || repr_str.chars().nth(0).unwrap().is_lowercase() {
            return Ok(PieceType::Pawn);
        }

        PieceType::from_alg(repr_str)
    }

    fn find_piece<'a>(&self, state: &'a State, piece_type: PieceType) -> Option<&'a Position> {
        let piece_positions = state.board.piece_positions_for(state.active_color);

        for position in piece_positions {
            let piece = state.board[&position].as_ref().unwrap();

            if piece.piece_type == piece_type {
                return Some(position);
            }
        }

        None
    }
}

impl<T: AsRef<str>> ToAlg for T {
    fn to_alg(&self) -> AlgNotation {
        AlgNotation::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::model::Piece;

    #[test]
    fn test_parse_move() {
        let state = State::default();

        let push_kings_pawn = "e4".to_alg().parse_move(&state).unwrap();
        assert_eq!(push_kings_pawn.piece, Piece::new(Color::White, PieceType::Pawn));
        assert_eq!(push_kings_pawn.from, Position::new(1, 4));
        assert_eq!(push_kings_pawn.to, Position::new(3, 4));

        let develop_knight = "Nc3".to_alg().parse_move(&state).unwrap();
        assert_eq!(develop_knight.piece, Piece::new(Color::White, PieceType::Knight));
        assert_eq!(develop_knight.from, Position::new(0, 1));
        assert_eq!(develop_knight.to, Position::new(2, 2));
    }
}
