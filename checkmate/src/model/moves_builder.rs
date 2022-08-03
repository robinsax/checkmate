use bitmask_enum::bitmask;

use super::position::Position;
use super::piece::Piece;
use super::move_repr::Move;
use super::state::State;

//  TODO: Refactor.

#[bitmask(u8)]
pub(super) enum MoveCondition {
    IsAttack,
    IsNotAttack
}

pub(super) struct MovesBuilder<'t> {
    state: &'t State,
    piece: &'t Piece,
    from: &'t Position,
    moves: Vec<Move>
}

impl<'t> MovesBuilder<'t> {
    pub fn new(state: &'t State, from: &'t Position) -> Self {
        Self{
            state, from,
            piece: state.board[from].as_ref().unwrap(),
            moves: Vec::new()
        }
    }

    pub fn push_if_valid(&mut self, position: &Position) -> Option<&Move> {
        if !position.is_valid() {
            return None;
        }

        let dest_position = &self.state.board[position];
        let is_occupied = dest_position.is_some();

        if is_occupied {
            let is_occupant_ally = match dest_position {
                Some(occupant) => occupant.color == self.piece.color,
                None => false
            };
            if is_occupant_ally {
                return None;
            }
        }

        let result_move = Move::new(
            self.from.clone(), position.clone(), self.piece.clone(),
            dest_position.clone(), None, None
        );

        let next_state = self.state.next_for_move(&result_move);
        if next_state.is_check_against(self.piece.color) {
            return None;
        }

        Some(self.push(result_move))
    }

    pub fn push(&mut self, safe_move: Move) -> &Move {
        self.moves.push(safe_move);

        &self.moves.last().unwrap()
    }

    pub fn push_while_valid(&mut self, direction_fn: fn (from: Position) -> Position) {
        let mut cur_position = direction_fn(self.from.clone());

        loop {
            match self.push_if_valid(&cur_position) {
                Some(result_move) => {
                    if result_move.taken.is_some() {
                        return
                    }
                },
                None => return,
            }

            cur_position = direction_fn(cur_position);
        } 
    }

    pub fn build(self) -> Vec<Move> {
        self.moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_with_invalid() {
        let state = State::default();
        let position = Position::new(0, 0);
        let mut builder = MovesBuilder::new(&state, &position);

        builder.push_if_valid(&Position::new(0, 1));

        assert_eq!(builder.build().len(), 0);
    }

    #[test]
    fn test_push_with_valid() {
        let state = State::default();
        let position = Position::new(1, 2);
        let mut builder = MovesBuilder::new(&state, &position);

        builder.push_if_valid(&Position::new(2, 2));

        assert_eq!(builder.build().len(), 1);
    }
}
