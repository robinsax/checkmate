use super::color::Color;
use super::position::Position;
use super::move_repr::{Move, CastleMoves};
use super::state::State;
use super::board_builder::BoardBuilder;

pub struct StateBuilder {
    board: BoardBuilder,
    move_history: Vec<Move>,
    active_color: Option<Color>,
    allowed_castles: Option<[CastleMoves; 2]>,
    en_passant_target: Option<Option<Position>>,
}

impl StateBuilder {
    pub fn new() -> Self {
        Self{
            board: BoardBuilder::new(),
            move_history: Vec::new(),
            active_color: None,
            allowed_castles: None,
            en_passant_target: None
        }
    }

    pub fn board_builder(&mut self) -> &mut BoardBuilder {
        &mut self.board
    }

    pub fn push_history(&mut self, oldest_move: Move) {
        self.move_history.push(oldest_move);
    }

    pub fn set_abstract_history(
        &mut self,
        active_color: Color, allowed_castles: [CastleMoves; 2],
        en_passant_target: Option<Position>
    ) {
        self.active_color = Some(active_color);
        self.allowed_castles = Some(allowed_castles);
        self.en_passant_target = Some(en_passant_target);
    }

    fn infer_active_color(&self) -> Color {
        match self.move_history.last() {
            Some(m) => !m.piece.color,
            None => Color::White
        }
    }

    fn infer_allowed_castles(&self) -> [CastleMoves; 2] {
        let mut allowed = [CastleMoves::all(), CastleMoves::all()];

        for check_move in &self.move_history {
            if let Some(mask) = check_move.disallowed_castle() {
                let color_idx: usize = check_move.piece.color.into();

                allowed[color_idx] = allowed[color_idx].and(mask.not());   
            }
        }

        allowed
    }

    pub fn build(&self) -> State {
        let active_color = match self.active_color {
            Some(c) => c,
            None => self.infer_active_color()
        };
        let allowed_castles = match self.allowed_castles {
            Some(m) => m,
            None => self.infer_allowed_castles()
        };

        State::new(self.board.build(), active_color, allowed_castles, None, self.move_history.clone())
    }
}
