use std::fmt;

use regex::Regex;
use lazy_static::lazy_static;

use crate::model::{State, StateBuilder, Color, CastleMoves, Piece, Position};
use crate::errors::ValidationError;
use super::format::{ToState, ToPieceType, ToPiece};
use super::alg::ToAlg;

pub trait ToFEN {
    fn to_fen(&self) -> FENotation;
}

impl<T: AsRef<str>> ToFEN for T {
    fn to_fen(&self) -> FENotation {
        FENotation::new(self)
    }
}

pub struct FENotation(String);

impl FENotation {
    pub fn new(data: impl AsRef<str>) -> Self {
        Self(data.as_ref().to_string())
    }
}

impl fmt::Display for FENotation {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{}", self.0)
    }
}

impl ToPiece for FENotation {
    fn to_piece(self) -> Result<Piece, ValidationError> {
        let piece_char = match self.0.chars().nth(0) {
            Some(c) => c,
            None => return Err(ValidationError::Parse {token: self.0}),
        };

        let color = match piece_char.is_uppercase() {
            true => Color::White,
            false => Color::Black
        };
        let piece_type = piece_char.to_string().to_uppercase().to_alg().to_piece_type()?;

        Ok(Piece::new(color, piece_type))
    }
}

impl ToState for FENotation {
    fn to_state(self) -> Result<State, ValidationError> {
        lazy_static! {
            static ref STATE_RE: Regex = Regex::new(r"^([/rnbqkpRNBQKP0-9]+)\s([wb])\s([kqKQ]+)\s((?:[a-h][0-9])|-)\s([0-9]+)\s([0-9]+)$").unwrap();
        }

        let state_str = &self.0;
        let mut builder = StateBuilder::new();

        let captures = match STATE_RE.captures(state_str) {
            Some(m) => m,
            None => return Err(ValidationError::InvalidState{token: self.0}),
        };

        let positions_str = &captures[1];
        let active_color_str = &captures[2];
        let allowed_castles_str = &captures[3];
        let en_passant_str = &captures[4];
        let draw_clock_str = &captures[5];
        let turn_clock_str = &captures[6];

        let ranks_strs: Vec<&str> = positions_str.split("/").collect();
        if ranks_strs.len() != 8 {
            return Err(ValidationError::Parse{token: positions_str.to_owned()});
        }

        for (inv_rank_idx, rank_str) in ranks_strs.iter().enumerate() {
            let mut file_idx = 0;
            for placement_char in rank_str.chars() {
                let placement_str = placement_char.to_string();
                if let Ok(skip_count) = placement_str.parse::<usize>() {
                    file_idx += skip_count;
                    continue;
                }

                let position = Position::new(7 - inv_rank_idx, file_idx);
                let piece = placement_str.to_fen().to_piece()?;

                builder.board_builder().place_piece(piece, position);
                file_idx += 1;
            }
        }

        let active_color = match active_color_str {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(ValidationError::Parse{token: active_color_str.to_owned()}),
        };

        let mut allowed_castles = [CastleMoves::none(), CastleMoves::none()];
        for allowed_char in allowed_castles_str.chars() {
            let color_idx = match allowed_char.is_uppercase() {
                true => 0,
                false => 1
            };

            let which = match allowed_char.to_string().to_lowercase().as_str() {
                "k" => CastleMoves::KingSide,
                "q" => CastleMoves::QueenSide,
                _ => return Err(ValidationError::Parse{token: allowed_castles_str.to_owned()}),
            };

            allowed_castles[color_idx] = allowed_castles[color_idx].or(which);
        }

        //  TODO: En-passant.
        builder.set_abstract_history(active_color, allowed_castles, None);

        Ok(builder.build())
    }
}

impl ToFEN for Color {
    fn to_fen(&self) -> FENotation {
        match *self {
            Color::White => "w",
            Color::Black => "b"
        }.to_fen()
    }
}

impl ToFEN for State {
    fn to_fen(&self) -> FENotation {
        let mut ranks_strs: Vec<String> = Vec::new();

        for rank_idx in (0..8).rev() {
            let mut rank_strs: Vec<String> = Vec::new();
            let mut empties = 0;

            for file_idx in 0..8 {
                let position = Position::new(rank_idx, file_idx);

                if let Some(piece) = self.board[&position].as_ref() {
                    if empties > 0 {
                        rank_strs.push(format!("{}", empties));
                        empties = 0;
                    }

                    let mut type_str = piece.piece_type.to_alg().to_string();
                    if piece.color == Color::Black {
                        type_str = type_str.to_lowercase();
                    }
                    rank_strs.push(type_str);
                    continue;
                }

                empties += 1;
            }

            if empties > 0 {
                rank_strs.push(format!("{}", empties));
            }
            ranks_strs.push(rank_strs.join(""));
        }

        let mut allowed_castles_strs: Vec<String> = Vec::new();
        let mut add_castles = |color: Color| {
            let mask = self.get_allowed_castles(color);

            if mask.contains(CastleMoves::KingSide) {
                let mut mask_char = "k".to_string();
                if color == Color::White {
                    mask_char = mask_char.to_uppercase();
                }

                allowed_castles_strs.push(mask_char);
            }
            if mask.contains(CastleMoves::QueenSide) {
                let mut mask_char = "q".to_string();
                if color == Color::White {
                    mask_char = mask_char.to_uppercase();
                }

                allowed_castles_strs.push(mask_char);
            }
        };
        add_castles(Color::White);
        add_castles(Color::Black);

        let mut en_passant_str = "-".to_string();
        if let Some(en_passant_target) = self.get_en_passant_position() {
            en_passant_str = en_passant_target.to_alg().to_string();
        }

        // TODO: Clocks.
        format!(
            "{} {} {} {} 0 {}",
            ranks_strs.join("/"), self.active_color.to_fen(), allowed_castles_strs.join(""),
            en_passant_str, (self.move_history.len() / 2) + 1
        ).to_fen()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::PieceType;

    #[test]
    fn test_parse_state() {
        let mut state = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_fen().to_state().unwrap();

        assert_eq!(state.active_color, Color::White);
        assert_eq!(state.board[&Position::new(1, 2)].as_ref().unwrap(), &Piece::new(Color::White, PieceType::Pawn));

        state = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b Qk e3 0 1".to_fen().to_state().unwrap();

        assert_eq!(state.board[&Position::new(3, 4)].as_ref().unwrap(), &Piece::new(Color::White, PieceType::Pawn));
        assert_eq!(state.active_color, Color::Black);

        assert_eq!(state.get_allowed_castles(Color::Black), CastleMoves::KingSide);
        assert_eq!(state.get_allowed_castles(Color::White), CastleMoves::QueenSide);
    }

    #[test]
    fn test_write_state() {
        let mut state = State::default();

        assert_eq!(state.to_fen().to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let moves = state.get_legal_moves();
        let next_move = moves.iter().find(|m| m.to == Position::new(3, 2)).unwrap();
        state = state.next_for_move(next_move);

        assert_eq!(state.to_fen().to_string(), "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq c3 0 1");
    }
}
