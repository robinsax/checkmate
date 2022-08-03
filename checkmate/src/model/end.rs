use readonly;

pub use super::color::Color;

#[derive(Debug, PartialEq)]
pub enum EndCondition {
    Checkmate,
    Stalemate,
    InsufficientMateriel,
    Surrender
}

#[readonly::make]
#[derive(Debug, PartialEq)]
pub struct EndResult {
    pub condition: EndCondition,
    pub winner: Option<Color>
}

impl EndResult {
    pub fn win(winner: Color, condition: EndCondition) -> Self {
        Self{
            winner: Some(winner),
            condition
        }
    }

    pub fn draw(condition: EndCondition) -> Self {
        Self{
            winner: None,
            condition
        }
    }
}
