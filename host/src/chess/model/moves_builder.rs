use bitmask_enum::bitmask;

use super::position::Position;
use super::piece::Piece;
use super::move_repr::Move;
use super::state::State;

#[bitmask(u8)]
pub(super) enum MoveCondition {
    IsAttack,
    IsNotAttack,
    NoHistory
}

pub(super) struct MovesBuilder<'t> {
    state: &'t State,
    piece: &'t Piece,
    from: &'t Position,
    moves: Vec<Move>,
    lookahead: bool
}

impl<'t> MovesBuilder<'t> {
    pub fn new(state: &'t State, from: &'t Position, lookahead: bool) -> Self {
        Self{
            state, from, lookahead,
            piece: state.board[from].as_ref().unwrap(),
            moves: Vec::new()
        }
    }

    pub fn push_if_valid(&mut self, position: &Position) -> Option<&Move> {
        self.push_if_valid_and(position, MoveCondition::none())
    }

    pub fn push_if_valid_and(&mut self, position: &Position, conds: MoveCondition) -> Option<&Move> {
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

        if conds.contains(MoveCondition::IsAttack) && !is_occupied {
            return None;
        }
        if conds.contains(MoveCondition::IsNotAttack) && is_occupied {
            return None;
        }
        if conds.contains(MoveCondition::NoHistory) && self.state.has_piece_at_moved(self.from) {
            return None;
        }

        let mut result_move = Move::new(self.from.clone(), position.clone(), self.piece.clone());
        if let Some(occupant) = dest_position {
            result_move.set_taken(occupant.clone());
        }

        if !self.lookahead {
            let next_state = self.state.next_for_move(&result_move);
            if next_state.is_check_against(self.piece.color) {
                return None;
            }
        }

        self.moves.push(result_move);
        Some(&self.moves.last().unwrap())
    }
    
    pub fn push_unchecked(&mut self, to: &Position, castle: &(Position, Position)) -> &Move {
        let mut safe_move = Move::new(self.from.clone(), to.clone(), self.piece.clone());

        safe_move.set_castle(castle.clone());

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

    pub fn build(&self) -> Vec<Move> {
        self.moves.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_with_invalid() {
        let state = State::default();
        let position = Position::from_alg("a1").unwrap();
        let mut builder = MovesBuilder::new(&state, &position, false);

        builder.push_if_valid(&Position::from_alg("a2").unwrap());

        assert_eq!(builder.build().len(), 0);
    }

    #[test]
    fn test_push_with_valid() {
        let state = State::default();
        let position = Position::from_alg("c2").unwrap();
        let mut builder = MovesBuilder::new(&state, &position, false);

        builder.push_if_valid(&Position::from_alg("c3").unwrap());

        assert_eq!(builder.build().len(), 1);
    }
}
