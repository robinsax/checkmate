use log::info;

use tokio::sync::Mutex;

use super::model::State;
use super::agent::ModelAgent;
use super::format::StateFormat;

pub struct Game {
    state: Mutex<State>,
    players: [Mutex<Box<dyn ModelAgent>>; 2]
}

impl Game {
    pub fn new(white_player: Mutex<Box<dyn ModelAgent>>, black_player: Mutex<Box<dyn ModelAgent>>) -> Self {
        Self{
            state: Mutex::new(State::create_initial()),
            players: [white_player, black_player]
        }
    }

    pub async fn get_state(&self) -> StateFormat {
        let state = self.state.lock().await;
        StateFormat::from_model(&state)
    }

    pub async fn tick(&self) {
        let state = (self.state.lock().await).clone();
        let agent_index = state.active_player.to_index();
        let agent = &mut self.players[agent_index].lock().await;

        info!("game_tick: block on agent");
        let next_move = &agent.get_move_for_model(&state).await;
        info!("{} {} {} -> {}", next_move.piece.color, next_move.piece.piece_type.to_char(), next_move.from, next_move.to);

        let mut state_lock = self.state.lock().await;
        *state_lock = state_lock.next_for_move(next_move);
    }
}
