use async_trait::async_trait;

use super::model::{State, Move};
use super::format::StateFormat;

#[async_trait]
pub trait ModelAgent: Send {
    async fn get_move_for_model(&mut self, state: &State) -> Move;
}

#[async_trait]
pub trait FormatAgent: Send {
    async fn get_move_for_format(&mut self, state: &StateFormat) -> String;
}

#[async_trait]
impl<T: FormatAgent + Sync> ModelAgent for T {
    async fn get_move_for_model(&mut self, state: &State) -> Move {
        let state_format = StateFormat::from_model(state);

        let fut = self.get_move_for_format(&state_format);
        let move_alg = fut.await;

        fn check_fmt(check: &Move) -> String {
            if let Some(promo) = &check.promotion {
                return format!("{}{}{}", check.from, check.to, promo.to_char());
            }

            format!("{}{}", check.from, check.to)
        }

        state.legal_moves().iter().find(|&check| {
            check_fmt(check) == *move_alg
        }).unwrap().clone()
    }
}
