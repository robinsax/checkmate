use std::fmt;

use super::super::errors::ValidationError;
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

impl fmt::Display for PieceType {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{}", self.to_alg())
    }
}

impl From<PieceType> for String {
    fn from(piece_type: PieceType) -> Self {
        let type_str = match piece_type {
            PieceType::Pawn => "P",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Knight => "N",
            PieceType::Queen => "Q",
            PieceType::King => "K"
        };

        type_str.to_string()
    }
}

impl PieceType {
    pub fn from_alg(alg: impl AsRef<str>) -> Result<Self, ValidationError> {
        let alg_str = alg.as_ref();
        if alg_str.len() != 1 {
            return Err(ValidationError::Parse{token: alg_str.to_owned()});
        }

        match alg_str.to_uppercase().as_ref() {
            "P" => Ok(PieceType::Pawn),
            "B" => Ok(PieceType::Bishop),
            "R" => Ok(PieceType::Rook),
            "N" => Ok(PieceType::Knight),
            "Q" => Ok(PieceType::Queen),
            "K" => Ok(PieceType::King),
            _ => Err(ValidationError::Parse{token: alg_str.to_owned()})
        }
    }

    pub fn to_alg(&self) -> String {
        let alg_str = match *self {
            PieceType::Pawn => "P",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Knight => "N",
            PieceType::Queen => "Q",
            PieceType::King => "K"
        };

        alg_str.to_string()
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alg_repr() {
        assert_eq!(PieceType::from_alg("p"), Ok(PieceType::Pawn));
        assert_eq!(PieceType::from_alg("n"), Ok(PieceType::Knight));
        assert_eq!(PieceType::from_alg("j"), Err(ValidationError::Parse{token: "j".to_owned()}));

        assert_eq!(PieceType::from_alg("p").unwrap().to_alg(), "P");
        assert_eq!(PieceType::from_alg("k").unwrap().to_alg(), "K");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", PieceType::Pawn), "P");
        assert_eq!(format!("{}", PieceType::Knight), "N");
    }

    #[test]
    fn test_materiel_value() {
        assert_eq!(PieceType::Pawn.materiel_value(), 1);
        assert_eq!(PieceType::Queen.materiel_value(), 9);
    }
}
