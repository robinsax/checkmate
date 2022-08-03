use readonly;
use log::info;

use super::color::Color;
use super::position::Position;
use super::piece_type::PieceType;
use super::move_repr::{Move, CastleMoves};
use super::board::Board;
use super::end::{EndResult, EndCondition};
use super::move_rules::compute_moves_for;

#[readonly::make]
#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub active_color: Color,
    pub move_history: Vec<Move>,
    en_passant_target: Option<Position>,
    allowed_castles: [CastleMoves; 2],
    lookahead: u32
}

impl Default for State {
    fn default() -> Self {
        Self{
            board: Board::default(),
            active_color: Color::White,
            move_history: Vec::new(),
            allowed_castles: [CastleMoves::all(), CastleMoves::all()],
            en_passant_target: None,
            lookahead: 0
        }
    }
}

impl State {
    pub(super) fn new(
        board: Board, active_color: Color,
        allowed_castles: [CastleMoves; 2],
        en_passant_target: Option<Position>,
        move_history: Vec<Move>
    ) -> Self {
        Self{
            board, move_history, active_color,
            allowed_castles, en_passant_target,
            lookahead: 0
        }
    }

    fn color_has_king_only(&self, color: Color) -> bool {
        let positions = self.board.positions_for(color);

        for position in positions {
            let piece = self.board[position].as_ref().unwrap();

            if piece.piece_type != PieceType::King {
                return false;
            }
        }

        true
    }

    pub fn check_result(&self) -> Option<EndResult> {
        if self.color_has_king_only(Color::White) && self.color_has_king_only(Color::Black) {
            return Some(
                EndResult::draw(EndCondition::InsufficientMateriel)
            );
        }

        let moves = self.get_legal_moves();
        if moves.len() == 0 {
            if self.is_check_against(self.active_color) {
                return Some(
                    EndResult::win(!self.active_color, EndCondition::Checkmate)
                );
            }

            return Some(
                EndResult::draw(EndCondition::Stalemate)
            );
        }

        None
    }

    pub fn get_en_passant_position(&self) -> Option<&Position> {
        self.en_passant_target.as_ref()
    }

    pub fn get_allowed_castles(&self, color: Color) -> CastleMoves {
        let color_idx: usize = color.into();

        self.allowed_castles[color_idx]
    }

    pub fn next_for_move(&self, next_move: &Move) -> State {
        let mut new_history = self.move_history.clone();
        new_history.push(next_move.clone());

        let new_board = self.board.next_for_move(
            &next_move.from, &next_move.to, &next_move.promotion,
            &next_move.castle
        );

        let color_idx: usize = next_move.piece.color.into();
        
        let mut new_allowed_castles = self.allowed_castles.clone();
        if let Some(disallowed) = next_move.disallowed_castle() {
            new_allowed_castles[color_idx] = new_allowed_castles[color_idx].and(disallowed.not());
        }

        let mut new_en_passant: Option<Position> = None;
        if next_move.piece.piece_type == PieceType::Pawn {
            let forward_one = next_move.from.forward(next_move.piece.color);
            if forward_one.forward(next_move.piece.color) == next_move.to {
                new_en_passant = Some(forward_one);
            }
        }

        State{
            board: new_board,
            move_history: new_history,
            active_color: !self.active_color,
            allowed_castles: new_allowed_castles,
            en_passant_target: new_en_passant,
            lookahead: if self.lookahead > 0 { self.lookahead + 1 } else { 0 }
        }
    }

    pub fn is_check_against(&self, color: Color) -> bool {
        if self.lookahead >= 2 {
            return false;
        }

        let mut lookahead = self.clone();
        lookahead.lookahead = 1;
        let check_moves = lookahead.get_legal_moves_for(!color);

        for check_move in check_moves {
            if let Some(piece) = &check_move.taken {
                if piece.piece_type == PieceType::King {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        self.get_legal_moves_for(self.active_color)
    }
    
    pub fn get_legal_moves_for(&self, color: Color) -> Vec<Move> {
        let piece_positions = self.board.positions_for(color);

        let mut moves: Vec<Move> = Vec::with_capacity(16);
        for position in piece_positions {
            moves.extend(compute_moves_for(self, &position));
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::piece::Piece;

    #[test]
    fn test_initial_state() {
        let state = State::default();

        assert_eq!(
            state.board[&Position::new(1, 4)],
            Some(Piece::new(Color::White, PieceType::Pawn))
        );
        assert_eq!(state.active_color, Color::White);
        assert_eq!(state.move_history.len(), 0);

        assert_eq!(state.get_legal_moves().len(), 20);
    }
}
