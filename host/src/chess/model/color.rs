use std::fmt::{ Display, Formatter, Result as FmtResult };

#[derive(PartialEq, Clone, Debug)]
pub enum Color {
    White,
    Black
}

impl Display for Color {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", if *self == Color::White { 'w' } else { 'b' })
    }
}

impl Color {
    pub fn to_index(&self) -> usize {
        if *self == Color::White { 0 } else { 1 }
    }

    pub(super) fn other(&self) -> Color {
        if *self == Color::White { Color::Black } else { Color::White }
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
    fn test_to_index() {
        assert_eq!(Color::White.to_index(), 0);
        assert_eq!(Color::Black.to_index(), 1);
    }

    #[test]
    fn test_other() {
        assert_eq!(Color::White.other(), Color::Black);
        assert_eq!(Color::Black.other(), Color::White);
    }
}