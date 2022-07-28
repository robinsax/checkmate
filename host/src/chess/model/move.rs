use readonly::{ make as make_readonly };

use super::position::{ Position };
use super::piece::{ PieceType, Piece };

#[make_readonly]
#[derive(Clone, Debug)]
pub struct Move {
    #[readonly]
    pub from: Position,
    #[readonly]
    pub to: Position,
    #[readonly]
    pub piece: Piece,
    #[readonly]
    pub taken: Option<Piece>,
    #[readonly]
    pub promotion: Option<PieceType>,
    #[readonly]
    pub castle: Option<(Position, Position)>
}

impl Move {
    pub fn new(from: Position, to: Position, piece: Piece) -> Self {
        Self{
            from, to, piece,
            taken: None,
            promotion: None,
            castle: None
        }
    }

    pub fn add_taken(&mut self, taken: Piece) {
        self.taken = Some(taken);
    }

    pub fn with_promotion(&mut self, promotion: PieceType) -> Move {
        let mut inst = self.clone();
        inst.promotion = Some(promotion);

        inst
    }

    pub fn add_castle(&mut self, castle: (Position, Position)) {
        self.castle = Some(castle);
    }
}
