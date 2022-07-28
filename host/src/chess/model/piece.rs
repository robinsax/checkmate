use readonly::{ make as make_readonly };

use super::color::{ Color };

#[derive(Clone, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Bishop,
    Rook,
    Knight,
    Queen,
    King
}

impl PieceType {
    pub fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Knight => 'N',
            PieceType::Queen => 'Q',
            PieceType::King => 'K'
        }
    }
}

#[make_readonly]
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
