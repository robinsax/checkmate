use std::fmt;

use regex::Regex;
use lazy_static::lazy_static;

use crate::model::{RANKS, FILES, State, Move, PieceType, Position};
use crate::errors::ValidationError;
use super::ToPosition;
use super::format::{ToMove, ToPieceType};

pub trait ToAlg {
    fn to_alg(&self) -> AlgNotation;
}

//  TODO: Over-copies?
impl<T: AsRef<str>> ToAlg for T {
    fn to_alg(&self) -> AlgNotation {
        AlgNotation::new(self)
    }
}

pub struct AlgNotation(String);

impl AlgNotation {
    pub fn new(data: impl AsRef<str>) -> Self {
        Self(data.as_ref().to_string())
    }
}

impl fmt::Display for AlgNotation {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{}", self.0)
    }
}

impl ToPosition for AlgNotation {
    fn to_position(self) -> Result<Position, ValidationError> {
        let alg_str = &self.0;
        if alg_str.len() != 2 {
            return Err(ValidationError::Parse{token: alg_str.to_owned()});
        }

        let alg_bytes = alg_str.as_bytes();
        let (rank_char, file_char) = (alg_bytes[1] as char, alg_bytes[0] as char);

        let rank = RANKS.iter().position(|c| *c == rank_char).ok_or(
            ValidationError::Parse{token: rank_char.to_string()},
        )?;
        let file = FILES.iter().position(|c| *c == file_char).ok_or(
            ValidationError::Parse{token: file_char.to_string()},
        )?;

        Ok(Position::new(rank, file))
    }
}

impl ToMove for AlgNotation {
    fn to_move(self, state: &State) -> Result<Move, ValidationError> {
        lazy_static! {
            static ref MOVE_RE: Regex = Regex::new(r"^([A-Z]|[a-h]?)(x?)([a-h][0-8])$").unwrap();
        }

        let move_str = &self.0;
        let moves = state.get_legal_moves();

        let king_side = move_str == "O-O";
        if king_side || move_str == "O-O-O" {
            return Err(ValidationError::InvalidState{token: move_str.to_owned()});
        }

        let captures = match MOVE_RE.captures(move_str) {
            Some(m) => m,
            None => return Err(ValidationError::Parse{token: move_str.to_owned()}),
        };

        let type_str = &captures[1];
        let capture_str = &captures[2];
        let dest_str = &captures[3];

        let dest_position = dest_str.to_alg().to_position()?;

        let is_capture = capture_str.len() > 0;
        if is_capture && state.board[&dest_position].as_ref().is_none() {
            return Err(ValidationError::InvalidState{token: dest_str.to_owned()})
        }

        let piece_type = match type_str.len() > 0 && type_str.chars().nth(0).unwrap().is_uppercase() {
            true => type_str.to_alg().to_piece_type()?,
            false => PieceType::Pawn
        };

        for check_move in moves {
            if check_move.to != dest_position {
                continue;
            }
            if check_move.piece.piece_type != piece_type {
                continue;
            }
            if piece_type == PieceType::Pawn && is_capture && check_move.from.rank_char().to_string() != type_str {
                continue;
            }

            return Ok(check_move.clone());
        }

        Err(ValidationError::InvalidState{token: move_str.to_owned()})
    }
}

impl ToPieceType for AlgNotation {
    fn to_piece_type(self) -> Result<PieceType, ValidationError> {
        match self.0.as_ref() {
            "P" => Ok(PieceType::Pawn),
            "B" => Ok(PieceType::Bishop),
            "R" => Ok(PieceType::Rook),
            "N" => Ok(PieceType::Knight),
            "Q" => Ok(PieceType::Queen),
            "K" => Ok(PieceType::King),
            _ => Err(ValidationError::Parse{token: self.0})
        }
    }
}

impl ToAlg for Move {
    fn to_alg(&self) -> AlgNotation {
        //  TODO: Disambiguation, castles, check, mate.
        let type_str = match self.piece.piece_type {
            PieceType::Pawn => "".to_string(),
            other => other.to_alg().to_string(),
        };
        
        let capture_str = match self.taken {
            Some(_) => "x",
            None => ""
        };

        let to_position_str = self.to.to_alg().to_string();

        format!("{}{}{}", type_str, capture_str, to_position_str).to_alg()
    }
}

impl ToAlg for PieceType {
    fn to_alg(&self) -> AlgNotation {
        match *self {
            PieceType::Pawn => "P",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Knight => "N",
            PieceType::Queen => "Q",
            PieceType::King => "K"
        }.to_alg()
    }
}

impl ToAlg for Position {
    fn to_alg(&self) -> AlgNotation {
        AlgNotation::new(format!("{}{}", self.file_char(), self.rank_char()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Piece, Color};

    #[test]
    fn test_parse_move() {
        let state = State::default();

        let push_kings_pawn = "e4".to_alg().to_move(&state).unwrap();
        assert_eq!(push_kings_pawn.piece, Piece::new(Color::White, PieceType::Pawn));
        assert_eq!(push_kings_pawn.from, Position::new(1, 4));
        assert_eq!(push_kings_pawn.to, Position::new(3, 4));

        let develop_knight = "Nc3".to_alg().to_move(&state).unwrap();
        assert_eq!(develop_knight.piece, Piece::new(Color::White, PieceType::Knight));
        assert_eq!(develop_knight.from, Position::new(0, 1));
        assert_eq!(develop_knight.to, Position::new(2, 2));
    }
}
