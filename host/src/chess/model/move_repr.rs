use std::fmt;

use readonly;

use super::position::Position;
use super::piece::{PieceType, Piece};

#[readonly::make]
#[derive(Clone, Debug)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub piece: Piece,
    pub taken: Option<Piece>,
    pub promotion: Option<PieceType>,
    pub castle: Option<(Position, Position)>
}

impl fmt::Display for Move {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{}{}{}", self.piece, self.from, self.to)
    }
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

    pub fn set_taken(&mut self, taken: Piece) {
        self.taken = Some(taken);
    }

    pub fn set_castle(&mut self, castle: (Position, Position)) {
        self.castle = Some(castle);
    }

    pub fn as_promotions(&mut self, promotions: &[PieceType]) -> Vec<Move> {
        let mut variants = Vec::with_capacity(promotions.len());
        
        for piece_type in promotions {
            let mut variant = self.clone();
            variant.promotion = Some(*piece_type);

            variants.push(variant);
        }

        variants
    }
}
