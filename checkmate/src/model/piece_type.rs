#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PieceType {
    Pawn,
    Bishop,
    Rook,
    Knight,
    Queen,
    King
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_materiel_value() {
        assert_eq!(PieceType::Pawn.materiel_value(), 1);
        assert_eq!(PieceType::Queen.materiel_value(), 9);
    }
}
