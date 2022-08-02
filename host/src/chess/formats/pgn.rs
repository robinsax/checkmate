use super::super::model::{Game, Move};
use super::super::errors::ValidationError;
use super::format::{GameFormat, MoveFormat};

pub struct PGNNotationFormat;

impl MoveFormat for PGNNotationFormat {
    fn parse_move(&self, game: &Game, data: impl AsRef<str>) -> Result<&Move, ValidationError> {

    }
}

impl GameFormat for PGNNotationFormat {
    fn parse_game(&self, data: impl AsRef<str>) -> Result<Game, ValidationError> {
        
    }
}

impl PGNNotationFormat {
    fn 
}