use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black
}

impl fmt::Display for Color {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color_char: char = (*self).into();

        write!(dest, "{}", color_char)
    }
}

impl ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self == Color::White { Color::Black } else { Color::White }
    }
}

impl From<Color> for char {
    fn from(color: Color) -> Self {
        if color == Color::White { 'w' } else { 'b' }
    }
}

impl From<Color> for String {
    fn from(color: Color) -> Self {
        let color_char: char = color.into();

        color_char.to_string()
    }
}

impl From<Color> for usize {
    fn from(color: Color) -> Self {
        if color == Color::White { 0 } else { 1 }
    }
}

impl From<usize> for Color {
    fn from(val: usize) -> Self {
        if val == 0 { Color::White } else { Color::Black }
    }
}

impl From<bool> for Color {
    fn from(flag: bool) -> Self {
        if flag { Color::White } else { Color::Black }
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
    
    #[test]
    fn test_casts() {
        assert_eq!(Into::<usize>::into(Color::White), 0);
        assert_eq!(Into::<usize>::into(Color::Black), 1);
        
        assert_eq!(Into::<Color>::into(false), Color::Black);
        assert_eq!(Into::<Color>::into(true), Color::White);
    }
}
