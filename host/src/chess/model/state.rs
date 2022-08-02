use readonly;

use super::color::Color;
use super::position::Position;
use super::piece_type::PieceType;
use super::piece::Piece;
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
    pub allowed_castles: [CastleMoves; 2],
    pub en_passant_target: Option<Position>,
    lookahead: bool
}

impl Default for State {
    fn default() -> Self {
        Self{
            board: Board::default(),
            active_color: Color::White,
            move_history: Vec::new(),
            allowed_castles: [CastleMoves::all(), CastleMoves::all()],
            en_passant_target: None,
            lookahead: false
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
            lookahead: false
        }
    }

    fn color_has_king_only(&self, color: Color) -> bool {
        let positions = self.board.piece_positions_for(color);

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

    pub fn next_for_move(&self, next_move: &Move) -> State {
        let mut new_history = self.move_history.clone();
        new_history.push(next_move.clone());

        let new_board = self.board.next_for_move(
            &next_move.from, &next_move.to, &next_move.promotion,
            &next_move.castle
        );

        State{
            board: new_board,
            move_history: new_history,
            active_color: !self.active_color,
            en_passant_target: None,
            allowed_castles: self.allowed_castles.clone(),
            lookahead: false
        }
    }

    pub fn is_check_against(&self, color: Color) -> bool {
        let check_moves = self.get_legal_moves_for_with_lookahead(!color, true);

        for check_move in check_moves {
            if let Some(piece) = &check_move.taken {
                if piece.piece_type == PieceType::King {
                    return true;
                }
            }
        }

        false
    }

    pub fn has_piece_at_moved(&self, position: &Position) -> bool {
        for check in &self.move_history {
            if check.to == *position {
                return true;
            }
        }

        false
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        self.get_legal_moves_for(self.active_color)
    }
    
    pub fn get_legal_moves_for(&self, color: Color) -> Vec<Move> {
        self.get_legal_moves_for_with_lookahead(color, false)
    }

    fn get_legal_moves_for_with_lookahead(&self, color: Color, lookahead: bool) -> Vec<Move> {
        let piece_positions = self.board.piece_positions_for(color);

        let mut moves: Vec<Move> = Vec::with_capacity(10);

        for position in piece_positions {
            let piece = self.board[position].as_ref().unwrap();
            
            moves.extend(compute_moves_for(self, &position, &piece, lookahead));
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::piece::{ Piece };

    #[test]
    fn test_initial_state() {
        let state = State::default();

        assert_eq!(
            state.board[&Position::from_alg(&"e2".to_string()).unwrap()],
            Some(Piece::new(Color::White, PieceType::Pawn))
        );
        assert_eq!(state.active_color, Color::White);
        assert_eq!(state.move_history.len(), 0);

        assert_eq!(state.get_legal_moves().len(), 20);
    }
}
