use std::fmt;

use readonly;

use super::color::Color;
use super::super::errors::ValidationError;
use super::piece_type::PieceType;

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
    pub fn from_fen_component(fen: impl AsRef<str>) -> Result<Self, ValidationError> {
        let fen_str = fen.as_ref();
        if fen_str.len() != 1 {
            return Err(ValidationError::Parse{token: fen_str.to_owned()});
        }

        let piece_type = PieceType::from_alg(fen_str)?;
        let color: Color = fen_str.chars().nth(0).unwrap().is_uppercase().into();

        Ok(Piece::new(color, piece_type))
    }

    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self{color, piece_type}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_repr() {
        assert_eq!(Piece::from_fen_component("P"), Ok(Piece::new(Color::White, PieceType::Pawn)));
        assert_eq!(Piece::from_fen_component("r"), Ok(Piece::new(Color::Black, PieceType::Rook)));

        assert_eq!(Piece::from_fen_component("j"), Err(ValidationError::Parse{token: "j".to_owned()}));
    }
    
    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Piece::new(Color::Black, PieceType::Pawn)), "bP");
        assert_eq!(format!("{}", Piece::new(Color::White, PieceType::Rook)), "wR");
    }
}
