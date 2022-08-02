use std::fmt;

use regex::Regex;
use lazy_static::lazy_static;

use super::super::model::{State, StateBuilder, Move, Color, CastleMoves, Board, Piece, Position, PieceType};
use super::super::errors::ValidationError;
use super::format::{StateFormat, MoveFormat};

pub trait ToFEN {
    fn to_fen(&self) -> FENotation;
}

pub struct FENotation(String);

impl fmt::Display for FENotation {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{}", self.0)
    }
}

impl StateFormat for FENotation {
    fn parse_state(&self) -> Result<State, ValidationError> {
        lazy_static! {
            static ref STATE_RE: Regex = Regex::new(r"^([/rnbqkpRNBQKP0-9]+)\s([wb])\s([kqKQ]+)\s((?:[a-h][0-9])|-)\s([0-9]+)\s([0-9]+)$").unwrap();
        }

        let state_str = &self.0;
        let mut builder = StateBuilder::new();

        let captures = match STATE_RE.captures(state_str) {
            Some(m) => m,
            None => return Err(ValidationError::InvalidState{token: state_str.to_owned()}),
        };

        let positions_str = captures.get(1).unwrap().as_str();
        let active_color_str = captures.get(2).unwrap().as_str();
        let allowed_castles_str = captures.get(3).unwrap().as_str();
        let en_passant_str = captures.get(4).unwrap().as_str();
        let draw_clock_str = captures.get(5).unwrap().as_str();
        let turn_clock_str = captures.get(6).unwrap().as_str();

        let ranks_strs: Vec<&str> = positions_str.split("/").collect();
        if ranks_strs.len() != 8 {
            return Err(ValidationError::Parse{token: positions_str.to_owned()});
        }

        for (rank_idx, rank_str) in ranks_strs.iter().enumerate() {
            let mut file_idx = 0;
            for placement_char in rank_str.chars() {
                let placement_str = placement_char.to_string();
                if let Ok(skip_count) = placement_str.parse::<usize>() {
                    file_idx += skip_count;
                    continue;
                }

                let position = Position::new(rank_idx, file_idx);
                let piece_type = PieceType::from_alg(placement_str)?;
                let color = match placement_char.is_uppercase() {
                    true => Color::White,
                    false => Color::Black
                };

                builder.board_builder().place_piece(Piece::new(color, piece_type), position);
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

        //  TODO: en passant.
        builder.set_abstract_history(active_color, allowed_castles, None);

        Ok(builder.build())
    }
}

impl FENotation {
    pub fn new(data: impl AsRef<str>) -> Self {
        Self(data.as_ref().to_string())
    }
}

impl<T: AsRef<str>> ToFEN for T {
    fn to_fen(&self) -> FENotation {
        FENotation::new(self)
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

                    let mut type_str: String = piece.piece_type.into();
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

        let active_color_str: String = self.active_color.into();

        let mut allowed_castles_strs: Vec<String> = Vec::new();
        for (color_idx, mask) in self.allowed_castles.iter().enumerate() {
            let color: Color = color_idx.into();

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
        }

        let mut en_passant_str = "-".to_string();
        if let Some(en_passant_target) = &self.en_passant_target {
            en_passant_str = format!("{}", en_passant_target);
        }

        // TODO: Clocks.
        format!(
            "{} {} {} {} 0 1",
            ranks_strs.join("/"), active_color_str, allowed_castles_strs.join(""),
            en_passant_str
        ).to_fen()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state() {
        let state = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b Qk e3 0 1".to_fen().parse_state().unwrap();

        assert_eq!(state.board[&Position::new(4, 4)].as_ref().unwrap(), &Piece::new(Color::White, PieceType::Pawn));
        assert_eq!(state.active_color, Color::Black);

        assert_eq!(state.allowed_castles[Into::<usize>::into(Color::Black)], CastleMoves::KingSide);
        assert_eq!(state.allowed_castles[Into::<usize>::into(Color::White)], CastleMoves::QueenSide);
    }

    #[test]
    fn test_write_state() {
        let mut state = State::default();

        assert_eq!(state.to_fen().to_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let moves = state.get_legal_moves();
        let next_move = moves.iter().find(|m| m.to == Position::new(3, 2)).unwrap();
        state = state.next_for_move(next_move);

        assert_eq!(state.to_fen().to_string(), "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 1");
    }
}
