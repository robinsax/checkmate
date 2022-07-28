use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black
}

impl fmt::Display for Color {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(dest, "{}", match *self {
            Color::White => 'w',
            Color::Black => 'b'
        })
    }
}

impl ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self == Color::White { Color::Black } else { Color::White }
    }
}

impl From<Color> for usize {
    fn from(color: Color) -> Self {
        if color == Color::White { 0 } else { 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Color::White), "w");
        assert_eq!(format!("{}", Color::Black), "b");
    }

    #[test]
    fn test_not() {
        assert_eq!(!Color::White, Color::Black);
        assert_eq!(!Color::Black, Color::White);
    }
}
