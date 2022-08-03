use readonly;

use super::color::Color;

pub static RANKS: &'static [char] = &['1', '2', '3', '4', '5', '6', '7', '8'];
pub static FILES: &'static [char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

/// `Position` represents a board position without awareness of of underlying board.
/// Primarily used as an index key of [`Board`].
#[readonly::make]
#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub rank: usize,
    pub file: usize
}

impl Position {
    pub fn new(rank: usize, file: usize) -> Self {
        Self{rank, file}
    }

    pub fn is_valid(&self) -> bool {
        (self.rank < 8) && (self.file < 8)
    }

    pub fn file_char(&self) -> char {
        FILES[self.file]
    }
    
    pub fn rank_char(&self) -> char {
        RANKS[self.rank]
    }

    pub fn forward(&self, for_color: Color) -> Position {
        if for_color == Color::White { self.up() } else { self.down() }
    }

    pub fn up(&self) -> Position {
        Position{rank: self.rank + 1, file: self.file}
    }

    pub fn down(&self) -> Position {
        if self.rank == 0 {
            return Position{rank: self.rank + 128, file: self.file};
        }

        Position{rank: self.rank - 1, file: self.file}
    }

    pub fn right(&self) -> Position {
        Position{rank: self.rank, file: self.file + 1}
    }
    
    pub fn left(&self) -> Position {
        if self.file == 0 {
            return Position{rank: self.rank, file: self.file + 128};
        }

        Position{rank: self.rank, file: self.file - 1}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk() {
        let initial = Position::new(1, 1);

        assert_eq!(initial.left(), Position::new(1, 0));
        assert_eq!(initial.right(), Position::new(1, 2));
        assert_eq!(initial.up(), Position::new(2, 1));
        assert_eq!(initial.down(), Position::new(0, 1));
        assert_eq!(initial.forward(Color::White), Position::new(2, 1));
        assert_eq!(initial.forward(Color::Black), Position::new(0, 1));
    }

    #[test]
    fn test_validation() {
        let initial = Position::new(1, 1);

        assert_eq!(initial.is_valid(), true);
        assert_eq!(initial.left().left().is_valid(), false);
    }
}
