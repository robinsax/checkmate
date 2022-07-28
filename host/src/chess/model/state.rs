use readonly;

use super::color::Color;
use super::position::Position;
use super::piece::PieceType;
use super::r#move::Move;
use super::board::Board;
use super::end::{EndResult, EndCondition};
use super::move_rules::moves_for;

#[readonly::make]
#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub history: Vec<Move>,
    pub active_player: Color,
    lookahead: bool
}

impl State {
    pub fn create_initial() -> Self {
        Self{
            board: Board::create_initial(),
            history: Vec::new(),
            active_player: Color::White,
            lookahead: false
        }
    }

    fn color_has_king_only(&self, color: &Color) -> bool {
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
        if self.color_has_king_only(&Color::White) && self.color_has_king_only(&Color::Black) {
            return Some(
                EndResult::draw(EndCondition::InsufficientMateriel)
            );
        }

        let moves = self.legal_moves();
        if moves.len() == 0 {
            if self.is_check_against(&self.active_player) {
                return Some(
                    EndResult::win(&self.active_player.other(), EndCondition::Checkmate)
                );
            }

            return Some(
                EndResult::draw(EndCondition::Stalemate)
            );
        }

        None
    }

    pub fn next_for_move(&self, next_move: &Move) -> State {
        let mut new_history = self.history.clone();
        new_history.push(next_move.clone());

        let new_board = self.board.next_for_move(
            &next_move.from, &next_move.to, &next_move.promotion,
            &next_move.castle
        );

        State{
            board: new_board,
            history: new_history,
            active_player: self.active_player.other(),
            lookahead: false
        }
    }

    pub fn is_check_against(&self, color: &Color) -> bool {
        let check_moves = self.legal_moves_for_with_options(&color.other(), true);

        for check_move in check_moves {
            if let Some(piece) = &check_move.taken {
                if piece.piece_type == PieceType::King {
                    return true;
                }
            }
        }

        false
    }

    pub fn move_escapes_check(&self, check_move: &Move) -> bool {
        let next = self.next_for_move(check_move);

        !next.is_check_against(&self.active_player)
    }

    pub fn has_piece_at_moved(&self, position: &Position) -> bool {
        for check in &self.history {
            if check.to == *position {
                return true;
            }
        }

        false
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        self.legal_moves_for(&self.active_player)
    }
    
    pub fn legal_moves_for(&self, color: &Color) -> Vec<Move> {
        self.legal_moves_for_with_options(color, false)
    }

    fn legal_moves_for_with_options(&self, color: &Color, lookahead: bool) -> Vec<Move> {
        let piece_positions = self.board.piece_positions_for(color);

        let mut moves: Vec<Move> = Vec::with_capacity(10);

        for position in piece_positions {
            let piece = self.board[position].as_ref().unwrap();
            
            moves.extend(moves_for(self, &position, &piece, lookahead));
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
        let state = State::create_initial();

        assert_eq!(
            state.board[&Position::from_alg("e2".to_string()).unwrap()],
            Some(Piece::new(Color::White, PieceType::Pawn))
        );
        assert_eq!(state.active_player, Color::White);
        assert_eq!(state.history.len(), 0);

        assert_eq!(state.legal_moves().len(), 20);
    }
}
