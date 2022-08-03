use std::fmt;

use regex::Regex;
use lazy_static::lazy_static;

use crate::formats::ToMove;
use crate::model::{State};
use crate::errors::ValidationError;
use super::format::ToState;
use super::alg::ToAlg;

pub trait ToPGN {
    fn to_pgn(&self) -> PGNotation;
}

impl<T: AsRef<str>> ToPGN for T {
    fn to_pgn(&self) -> PGNotation {
        PGNotation::new(self)
    }
}

pub struct PGNotation(String);

impl PGNotation {
    pub fn new(data: impl AsRef<str>) -> Self {
        Self(data.as_ref().to_string())
    }
}

impl fmt::Display for PGNotation {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{}", self.0)
    }
}

impl ToState for PGNotation {
    fn to_state(self) -> Result<State, ValidationError> {
        lazy_static! {
            static ref TURN_RE: Regex = Regex::new(r"([0-9])+\.\s([^\s]+)\s([^\s]+)").unwrap();
        }

        let mut state = State::default();

        for captures in TURN_RE.captures_iter(&self.0) {
            let white_move_str = &captures[2];
            let black_move_str = &captures[3];

            state = state.next_for_move(&white_move_str.to_alg().to_move(&state)?);
            state = state.next_for_move(&black_move_str.to_alg().to_move(&state)?);
        }

        Ok(state)
    }
}

impl ToPGN for State {
    fn to_pgn(&self) -> PGNotation {
        let mut turn_strs: Vec<String> = Vec::new();

        let mut moves_iter = self.move_history.iter();
        loop {
            let white_move_str = match moves_iter.next() {
                Some(m) => m.to_alg().to_string(),
                None => break,
            };
            
            let mut ended = false;
            let black_move_str = match moves_iter.next() {
                Some(m) => m.to_alg().to_string(),
                None => {
                    ended = true;
                    "...".to_string()
                }
            };

            turn_strs.push(format!(
                "{}. {} {}",
                turn_strs.len() + 1, white_move_str, black_move_str 
            ));
            
            if ended {
                break;
            }
        }

        turn_strs.join(" ").to_pgn()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{Position, Piece, Color, PieceType};
    use super::*;

    #[test]
    fn test_parse_state() {
        let state = "1. e4 e5 2. Nf3 Nc6".to_pgn().to_state().unwrap();

        assert_eq!(state.board[Position::new(3, 4)].as_ref().unwrap(), &Piece::new(Color::White, PieceType::Pawn));
        assert_eq!(state.board[Position::new(4, 4)].as_ref().unwrap(), &Piece::new(Color::Black, PieceType::Pawn));
        assert_eq!(state.board[Position::new(2, 5)].as_ref().unwrap(), &Piece::new(Color::White, PieceType::Knight));
        assert_eq!(state.board[Position::new(5, 2)].as_ref().unwrap(), &Piece::new(Color::Black, PieceType::Knight));
    }
}
