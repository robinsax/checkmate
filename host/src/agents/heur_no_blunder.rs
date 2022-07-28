use async_trait::async_trait;

use crate::chess::model::{PieceType, State, Move};
use crate::chess::agent::ModelAgent;

#[derive(Clone)]
pub struct HeurNoBlunderAgent;

#[async_trait]
impl ModelAgent for HeurNoBlunderAgent {
    async fn get_move_for_model(&mut self, state: &State) -> Move {
        let mut loss_min = PieceType::Queen.materiel_value() + 1;
        let mut best_play: Option<&Move> = None;

        let moves = &state.get_legal_moves()[..];
        for check_move in moves {
            let next_state = state.next_for_move(check_move);

            let mut loss_max: u8 = 0;
            let moves_against = next_state.get_legal_moves();
            for attack_check in moves_against {
                if let Some(piece) = &attack_check.taken {
                    loss_max = piece.piece_type.materiel_value();
                }
            }

            if loss_max < loss_min {
                loss_min = loss_max;
                best_play = Some(check_move);
            }
        }

        best_play.unwrap().clone()
    }
}

impl HeurNoBlunderAgent {
    pub fn new() -> Self {
        Self{}
    }
}
