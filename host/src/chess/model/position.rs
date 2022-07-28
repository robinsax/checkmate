use std::fmt;

use readonly;

use super::errors::ValidationError;
use super::color::Color;

static RANKS: &'static [char] = &['1', '2', '3', '4', '5', '6', '7', '8'];
static FILES: &'static [char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

#[readonly::make]
#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub rank: usize,
    pub file: usize
}

impl fmt::Display for Position {
    fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_valid() {
            return write!(dest, "<invalid>");
        }

        write!(dest, "{}{}", FILES[self.file], RANKS[self.rank])
    }
}

impl Position {
    pub fn from_alg(alg: &String) -> Result<Self, ValidationError> {
        if alg.len() != 2 {
            return Err(ValidationError::Parse{token: alg.to_owned()});
        }

        let alg_bytes = alg.as_bytes();
        let (rank_char, file_char) = (alg_bytes[1] as char, alg_bytes[0] as char);

        let rank = RANKS.iter().position(|c| *c == rank_char).ok_or(
            ValidationError::Parse{token: rank_char.to_string()},
        )?;
        let file = FILES.iter().position(|c| *c == file_char).ok_or(
            ValidationError::Parse{token: file_char.to_string()},
        )?;

        Ok(Self::new(rank, file))
    }

    pub fn new(rank: usize, file: usize) -> Self {
        Self{rank, file}
    }

    pub fn is_valid(&self) -> bool {
        (self.rank < 8) && (self.file < 8)
    }

    pub fn is_end_rank(&self) -> bool {
        (self.rank == 7) || (self.rank == 0)
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
    fn test_from_alg() {
        assert_eq!(Position::from_alg("a1".to_owned()), Ok(Position::new(0, 0)));
        assert_eq!(Position::from_alg("e8".to_owned()), Ok(Position::new(7, 4)));
    }

    #[test]
    fn test_walk() {
        let initial = Position::from_alg("b2".to_owned()).unwrap();

        assert_eq!(initial.left(), Position::new(1, 0));
        assert_eq!(initial.right(), Position::new(1, 2));
        assert_eq!(initial.up(), Position::new(2, 1));
        assert_eq!(initial.down(), Position::new(0, 1));
    }

    #[test]
    fn test_validation() {
        let initial = Position::from_alg("b2".to_owned()).unwrap();

        assert_eq!(initial.is_valid(), true);
        assert_eq!(initial.left().left().is_valid(), false);
    }
}
