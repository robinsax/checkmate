use readonly::{ make as make_readonly };

use std::fmt::{ Display, Formatter, Result as FmtResult };

static RANKS: &'static [char] = &['1', '2', '3', '4', '5', '6', '7', '8'];
static FILES: &'static [char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub fn file_char(index: usize) -> char {
    FILES[index]
}

pub fn rank_char(index: usize) -> char {
    RANKS[index]
}

#[make_readonly]
#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    pub(super) rank: usize,
    pub(super) file: usize
}

impl Display for Position {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        if !self.is_valid() {
            return write!(formatter, "<invalid>");
        }

        write!(formatter, "{}{}", FILES[self.file], RANKS[self.rank])
    }
}

impl Position {
    pub fn from_chars(rank: char, file: char) -> Result<Self, &'static str> {
        let rank = RANKS.iter().position(|c| c == &rank).ok_or("invalid rank")?;
        let file = FILES.iter().position(|c| c == &file).ok_or("invalid file")?;

        Ok(Self::new(rank, file))
    }

    pub fn from_alg(alg: String) -> Result<Self, &'static str> {
        if alg.len() < 2 {
            return Err("invalid alg position: too short");
        }
        let alg_bytes = alg.as_bytes();

        Self::from_chars(alg_bytes[1] as char, alg_bytes[0] as char)
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
    fn test_from_chars() {
        assert_eq!(Position::from_chars('1', 'a'), Ok(Position::new(0, 0)));
        assert_eq!(Position::from_chars('8', 'e'), Ok(Position::new(7, 4)));
        assert_eq!(Position::from_chars('e', 'e'), Err("invalid rank"));
        assert_eq!(Position::from_chars('8', '8'), Err("invalid file"));
    }
    
    #[test]
    fn test_from_alg() {
        assert_eq!(Position::from_alg("a1".to_string()), Ok(Position::new(0, 0)));
        assert_eq!(Position::from_alg("e8".to_string()), Ok(Position::new(7, 4)));
    }

    #[test]
    fn test_walk() {
        let initial = Position::from_alg("b2".to_string()).unwrap();

        assert_eq!(initial.left(), Position::new(1, 0));
        assert_eq!(initial.right(), Position::new(1, 2));
        assert_eq!(initial.up(), Position::new(2, 1));
        assert_eq!(initial.down(), Position::new(0, 1));
    }

    #[test]
    fn test_validation() {
        let initial = Position::from_alg("b2".to_string()).unwrap();

        assert_eq!(initial.left().left().is_valid(), false);
    }
}
