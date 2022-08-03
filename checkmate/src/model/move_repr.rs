use readonly;
use bitmask_enum::bitmask;

use super::position::Position;
use super::piece_type::PieceType;
use super::piece::Piece;

#[bitmask(u8)]
#[derive(Clone, Copy)]
pub enum CastleMoves {
    KingSide,
    QueenSide
}

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

impl Move {
    pub(super) fn new(
        from: Position, to: Position, piece: Piece,
        taken: Option<Piece>, promotion: Option<PieceType>,
        castle: Option<(Position, Position)>
    ) -> Self {
        Self{from, to, piece, taken, promotion, castle}
    }

    pub fn with_promotion(&self, promotion: PieceType) -> Move {
        let mut variant = self.clone();
        variant.promotion = Some(promotion);

        variant
    }

    pub fn disallowed_castle(&self) -> Option<CastleMoves> {
        if let Some(castle) = &self.castle {
            return Some(match castle.0.file > castle.1.file {
                true => CastleMoves::KingSide,
                false => CastleMoves::QueenSide
            });
        }

        None
    }
}
