use std::fmt;
use std::ops;

use super::color::Color;
use super::position::Position;
use super::piece_type::PieceType;
use super::piece::Piece;

#[derive(Clone)]
pub struct Board {
    positions: [[Option<Piece>; 8]; 8],
    piece_positions: [Vec<Position>; 2]
}

impl ops::Index<&Position> for Board {
    type Output = Option<Piece>;

    fn index(&self, pos: &Position) -> &Self::Output {
        if !pos.is_valid() {
            return &None;
        }

        &self.positions[pos.rank][pos.file]
    }
}

impl Board {
    pub fn empty() -> Self {
        Self{
            positions: Default::default(),
            piece_positions: Default::default()
        }
    }

    pub(super) fn new(positions: [[Option<Piece>; 8]; 8], piece_positions: [Vec<Position>; 2]) -> Self {
        Self{positions, piece_positions}
    }

    //  TODO: Rm.
    pub fn place_piece(&mut self, position: Position, piece: Piece) {
        let owner_idx: usize = piece.color.into();

        self.positions[position.rank][position.file] = Some(piece);
        self.piece_positions[owner_idx].push(position);
    }

    pub fn piece_positions_for(&self, color: Color) -> &Vec<Position> {
        let index: usize = color.into();
        &self.piece_positions[index]
    }

    fn apply_move(
        &mut self, from: &Position, to: &Position,
        promo: &Option<PieceType>, castle: &Option<(Position, Position)>
    ) {
        let mut moved_piece = self[from].as_ref().unwrap().clone();
        if let Some(new_type) = promo {
            moved_piece = Piece::new(moved_piece.color.clone(), new_type.clone());
        }

        if let Some(taken_piece) = self[to].as_ref() {
            let taken_color_idx: usize = taken_piece.color.into();

            self.piece_positions[taken_color_idx] = self.piece_positions[taken_color_idx]
                .drain(..).filter(|p| *p != *to).collect();
        }

        let color_index: usize = moved_piece.color.into();
        let mut position_set = self.piece_positions[color_index].clone();

        if let Some((castle_from, castle_to)) = castle {
            let rook = self.positions[castle_from.rank][castle_from.file]
                .as_ref().unwrap().clone();

            self.positions[castle_from.rank][castle_from.file] = None;
            self.positions[castle_to.rank][castle_to.file] = Some(rook.clone());
            
            position_set = position_set.drain(..).filter(|p| *p != *castle_from).collect();
            position_set.push(castle_to.clone());
        }

        self.positions[from.rank][from.file] = None;
        self.positions[to.rank][to.file] = Some(moved_piece.clone());

        position_set = position_set.drain(..).filter(|p| *p != *from).collect();
        position_set.push(to.clone());
        self.piece_positions[color_index] = position_set;
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
            board[&Position::from_alg("a1".to_string()).unwrap()],
            Some(Piece::new(Color::White, PieceType::Rook))
        );
        assert_eq!(
            board[&Position::from_alg("e5".to_string()).unwrap()],
            None
        );
    }
    
    #[test]
    fn test_piece_positions_for() {
        let board = Board::default();
        
        assert_eq!(
            board.piece_positions_for(Color::White).len(),
            16
        );
    }
    
    #[test]
    fn test_next_for_move() {
        let board = Board::default().next_for_move(
            &Position::from_alg("e2").unwrap(),
            &Position::from_alg("e4").unwrap(),
            &None, &None
        ).next_for_move(
            &Position::from_alg("f7").unwrap(),
            &Position::from_alg("f5").unwrap(),
            &None, &None
        ).next_for_move(
            &Position::from_alg("e4").unwrap(),
            &Position::from_alg("f5").unwrap(),
            &None, &None
        );

        assert_eq!(
            board[&Position::from_alg("e2").unwrap()],
            None
        );
        assert_eq!(
            board[&Position::from_alg("f5").unwrap()],
            Some(Piece::new(Color::White, PieceType::Pawn))
        );
        assert_eq!(
            board.piece_positions_for(Color::White).len(),
            16
        );
        assert_eq!(
            board.piece_positions_for(Color::Black).len(),
            15
        );
    }
}