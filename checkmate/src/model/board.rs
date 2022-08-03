use std::ops;

use super::color::Color;
use super::position::Position;
use super::piece_type::PieceType;
use super::piece::Piece;

/// `Board` encapsulates a single board state and provides methods for its comprehension.
///
/// Usually used as a member of [`State`].
#[derive(Clone)]
pub struct Board {
    positions: [[Option<Piece>; 8]; 8],
    color_positions: [Vec<Position>; 2]
}

impl ops::Index<&Position> for Board {
    type Output = Option<Piece>;

    /// Return the piece at the current position, if there is one.
    /// 
    /// ```
    /// use checkmate::model::{Board, Position, Piece, PieceType, Color};
    ///
    /// let board = Board::default();
    /// 
    /// assert_eq!(board[&Position::new(0, 0)], Some(Piece::new(Color::White, PieceType::Rook)))
    /// ```
    fn index(&self, position: &Position) -> &Self::Output {
        if !position.is_valid() {
            return &None;
        }

        &self.positions[position.rank][position.file]
    }
}

impl ops::Index<Position> for Board {
    type Output = Option<Piece>;

    /// Return the piece at the current position, if there is one.
    /// 
    /// **Note**: Using a reference as the index key is usually preferable.
    fn index(&self, position: Position) -> &Self::Output {
        &self[&position]
    }
}

impl Board {
    pub(super) fn new(positions: [[Option<Piece>; 8]; 8], color_positions: [Vec<Position>; 2]) -> Self {
        Self{positions, color_positions}
    }

    /// Return the positions occupied by pieces of the given [`Color`].
    pub fn positions_for(&self, color: Color) -> &Vec<Position> {
        let index: usize = color.into();
        &self.color_positions[index]
    }

    fn apply_move(
        &mut self, from: &Position, to: &Position,
        promo: &Option<PieceType>, castle: &Option<(Position, Position)>
    ) {
        //  Apply promotion.
        let mut moved_piece = self[from].as_ref().unwrap().clone();
        if let Some(new_type) = promo {
            moved_piece = Piece::new(moved_piece.color.clone(), new_type.clone());
        }

        //  Apply opponent piece take.
        if let Some(taken_piece) = self[to].as_ref() {
            let taken_color_idx: usize = taken_piece.color.into();

            self.color_positions[taken_color_idx] = self.color_positions[taken_color_idx]
                .drain(..).filter(|p| *p != *to).collect();
        }

        let color_index: usize = moved_piece.color.into();
        let mut position_set = self.color_positions[color_index].clone();

        //  Apply castle.
        if let Some((castle_from, castle_to)) = castle {
            let rook = self.positions[castle_from.rank][castle_from.file]
                .as_ref().unwrap().clone();

            self.positions[castle_from.rank][castle_from.file] = None;
            self.positions[castle_to.rank][castle_to.file] = Some(rook.clone());
            
            position_set = position_set.drain(..).filter(|p| *p != *castle_from).collect();
            position_set.push(castle_to.clone());
        }

        //  Apply move.
        self.positions[from.rank][from.file] = None;
        self.positions[to.rank][to.file] = Some(moved_piece.clone());

        position_set = position_set.drain(..).filter(|p| *p != *from).collect();
        position_set.push(to.clone());
        self.color_positions[color_index] = position_set;
    }

    pub(super) fn next_for_move(
        &self, from: &Position, to: &Position,
        promo: &Option<PieceType>, castle: &Option<(Position, Position)>
    ) -> Board {
        let mut inst = self.clone();
        inst.apply_move(from, to, promo, castle);

        inst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexing() {
        let board = Board::default();
        
        assert_eq!(
            board[Position::new(0, 0)],
            Some(Piece::new(Color::White, PieceType::Rook))
        );
        assert_eq!(
            board[Position::new(4, 4)],
            None
        );
    }
    
    #[test]
    fn test_piece_positions_for() {
        let board = Board::default();
        
        assert_eq!(
            board.positions_for(Color::White).len(),
            16
        );
    }
    
    #[test]
    fn test_next_for_move() {
        let board = Board::default().next_for_move(
            &Position::new(1, 4),
            &Position::new(3, 4),
            &None, &None
        ).next_for_move(
            &Position::new(6, 5),
            &Position::new(4, 5),
            &None, &None
        ).next_for_move(
            &Position::new(3, 4),
            &Position::new(4, 5),
            &None, &None
        );

        assert_eq!(board[&Position::new(1, 4)], None);
        assert_eq!(
            board[&Position::new(4, 5)],
            Some(Piece::new(Color::White, PieceType::Pawn))
        );
        assert_eq!(board.positions_for(Color::White).len(), 16);
        assert_eq!(board.positions_for(Color::Black).len(), 15);
    }
}