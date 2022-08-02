use super::super::model::{Game, Move};
use super::super::errors::ValidationError;
use super::format::{GameFormat, MoveFormat};

pub struct UCINotationFormat;

impl MoveFormat for UCINotationFormat {
    fn parse_move(&self, game: &Game, data: impl AsRef<str>) -> Result<&Move, ValidationError> {
        
    }
}
