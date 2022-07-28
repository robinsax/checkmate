use std::fmt::{ Display, Formatter, Result as FmtResult };
use std::ops::{ Index };

use super::color::{ Color };
use super::position::{ Position, file_char };
use super::piece::{ Piece, PieceType };

#[derive(Clone)]
pub struct Board {
    positions: [[Option<Piece>; 8]; 8],
    piece_positions: [Vec<Position>; 2]
}

impl Index<&Position> for Board {
    type Output = Option<Piece>;

    fn index(&self, pos: &Position) -> &Self::Output {
        if !pos.is_valid() {
            return &None;
        }

        &self.positions[pos.rank][pos.file]
    }
}

impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        let mut repr = String::with_capacity(9 * 8);

        for rank in 0..8 {
            for file in 0..8 {
                repr.push(
                    match &self[&Position::new(rank, file)] {
                        Some(piece) => piece.piece_type.to_char(),
                        None => match rank + file % 2 {
                            0 => 'x',
                            _ => ' '
                        }
                    }
                )
            }       
            
            repr.push('\n');
        }

        write!(formatter, "{}", repr)
    }
}

impl Board {
    fn new() -> Self {
        Self{
            positions: Default::default(),
            piece_positions: [Vec::new(), Vec::new()]
        }
    }

    pub fn create_initial() -> Self {
        let mut inst = Self::new();

        let mut create_rank = |color: Color, rank: char, pieces: &Vec<PieceType>| {
            for (i, piece) in pieces.iter().enumerate() {
                let position = Position::from_chars(rank, file_char(i)).unwrap();

                inst.positions[position.rank][position.file] = Some(Piece::new(color.clone(), piece.clone()));
                inst.piece_positions[color.to_index()].push(position);
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
        create_rank(Color::Black, '8', back_rank);
        create_rank(Color::Black, '7', pawn_rank);
        create_rank(Color::White, '2', pawn_rank);
        create_rank(Color::White, '1', back_rank);

        inst
    }

    pub fn piece_positions_for(&self, color: &Color) -> &Vec<Position> {
        &self.piece_positions[color.to_index()]
    }

    fn apply_move(
        &mut self, from: &Position, to: &Position, promo: &Option<PieceType>,
        castle: &Option<(Position, Position)>
    ) {
        let mut moved_piece = self[&from].as_ref().unwrap().clone();
        
        if let Some(taken_piece) = self[&to].as_ref() {
            let other_color_index = taken_piece.color.to_index();

            let mut other_position_set = self.piece_positions[other_color_index].clone();
            other_position_set.remove(
                other_position_set.iter().position(|p| *p == *to).unwrap()
            );

            self.piece_positions[other_color_index] = other_position_set;
        }

        if let Some(new_type) = promo {
            moved_piece = Piece::new(moved_piece.color.clone(), new_type.clone());
        }

        let color_index = moved_piece.color.to_index();
        let mut position_set = self.piece_positions[color_index].clone();

        if let Some((castle_from, castle_to)) = castle {
            let rook = self.positions[castle_from.rank][castle_from.file].as_ref().unwrap().clone();
            self.positions[castle_from.rank][castle_from.file] = None;
            self.positions[castle_to.rank][castle_to.file] = Some(rook.clone());
            
            position_set.remove(
                position_set.iter().position(|p| *p == *castle_from).unwrap()
            );
            position_set.push(castle_to.clone());
        }

        self.positions[from.rank][from.file] = None;
        self.positions[to.rank][to.file] = Some(moved_piece.clone());

        position_set.remove(
            position_set.iter().position(|p| *p == *from).unwrap()
        );
        position_set.push(to.clone());
        self.piece_positions[color_index] = position_set;
    }

    pub(super) fn next_for_move(
        &self, from: &Position, to: &Position, promo: &Option<PieceType>,
        castle: &Option<(Position, Position)>
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
            board.piece_positions_for(&Color::White).len(),
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
            board.piece_positions_for(&Color::White).len(),
            16
        );
        assert_eq!(
            board.piece_positions_for(&Color::Black).len(),
            15
        );
    }
}