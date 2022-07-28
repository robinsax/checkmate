use std::fmt;
use std::ops;

use super::color::Color;
use super::position::Position;
use super::piece::{Piece, PieceType};

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

impl fmt::Display for Board {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::with_capacity(9 * 8);

        for rank in 0..8 {
            for file in 0..8 {
                repr.push(
                    match &self[&Position::new(rank, file)] {
                        Some(piece) => (&piece.piece_type).into(),
                        None => match rank + file % 2 {
                            0 => 'x',
                            _ => ' '
                        }
                    }
                )
            }       
            
            repr.push('\n');
        }

        write!(dest, "{}", repr)
    }
}

impl Board {
    fn empty() -> Self {
        Self{
            positions: Default::default(),
            piece_positions: [Vec::new(), Vec::new()]
        }
    }

    pub fn initial() -> Self {
        let mut inst = Self::empty();

        let mut create_rank = |color: Color, rank: usize, pieces: &Vec<PieceType>| {
            for (i, piece) in pieces.iter().enumerate() {
                inst.positions[rank][i] = Some(Piece::new(color.clone(), piece.clone()));

                let color_idx: usize = color.into();
                inst.piece_positions[color_idx].push(Position::new(rank, i));
            }
        };

        let back_rank = &vec![
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook
        ];
        let pawn_rank = &vec![
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn,
            PieceType::Pawn
        ];
        create_rank(Color::Black, 7, back_rank);
        create_rank(Color::Black, 6, pawn_rank);
        create_rank(Color::White, 1, pawn_rank);
        create_rank(Color::White, 0, back_rank);

        inst
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
        let board = Board::create_initial();
        
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
        let board = Board::create_initial();
        
        assert_eq!(
            board.piece_positions_for(Color::White).len(),
            16
        );
    }
    
    #[test]
    fn test_next_for_move() {
        let board = Board::create_initial().next_for_move(
            &Position::from_alg("e2".to_string()).unwrap(),
            &Position::from_alg("e4".to_string()).unwrap()
        ).next_for_move(
            &Position::from_alg("f7".to_string()).unwrap(),
            &Position::from_alg("f5".to_string()).unwrap()
        ).next_for_move(
            &Position::from_alg("e4".to_string()).unwrap(),
            &Position::from_alg("f5".to_string()).unwrap()
        );

        assert_eq!(
            board[&Position::from_alg("e2".to_string()).unwrap()],
            None
        );
        assert_eq!(
            board[&Position::from_alg("f5".to_string()).unwrap()],
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