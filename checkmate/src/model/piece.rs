use readonly;

use super::color::Color;
use super::piece_type::PieceType;

#[readonly::make]
#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self{color, piece_type}
    }
}
