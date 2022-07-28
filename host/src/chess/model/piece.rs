use std::fmt;

use readonly;

use super::color::Color;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Bishop,
    Rook,
    Knight,
    Queen,
    King
}

impl From<&PieceType> for char {
    fn from(piece_type: &PieceType) -> char {
        match *piece_type {
            PieceType::Pawn => 'P',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Knight => 'N',
            PieceType::Queen => 'Q',
            PieceType::King => 'K'
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{}", Into::<char>::into(self))
    }
}

impl PieceType {
    pub fn materiel_value(&self) -> u8 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Bishop => 3,
            PieceType::Knight => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 0
        }
    }
}

#[readonly::make]
#[derive(Clone, Debug, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType
}

impl fmt::Display for Piece {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{}{}", self.color, self.piece_type)
    }
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self{color, piece_type}
    }
}
