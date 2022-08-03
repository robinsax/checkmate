use std::ops;

/// Binary `Color` encapsulation. Is [`Copy`] and should be used as such since
/// since pointers are larger than the u8 this compiles to.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black
}

impl ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}

impl From<Color> for usize {
    /// Return `0` for white and `1` for black. Useful for indexing per-`Color` data
    /// stored in slices.
    fn from(color: Color) -> Self {
        match color {
            Color::White => 0,
            Color::Black => 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not() {
        assert_eq!(!Color::White, Color::Black);
        assert_eq!(!Color::Black, Color::White);
    }
    
    #[test]
    fn test_cast_usize() {
        assert_eq!(Into::<usize>::into(Color::White), 0);
        assert_eq!(Into::<usize>::into(Color::Black), 1);
    }
}
