use log::info;

use tokio::sync::Mutex;

use crate::model::State;
use crate::agents::Agent;

pub struct Game {
    state: Mutex<State>,
    players: [Mutex<Box<dyn Agent>>; 2]
}

impl Game {
    pub fn new(white_player: Mutex<Box<dyn Agent>>, black_player: Mutex<Box<dyn Agent>>) -> Self {
        Self{
            state: Mutex::new(State::default()),
            players: [white_player, black_player]
        }
    }

    pub async fn tick(&self) {
        let state = (self.state.lock().await).clone();

        let agent_idx: usize = state.active_color.into();
        let agent = &mut self.players[agent_idx].lock().await;

        info!("game_tick: block on agent");
        let next_move = &agent.get_move_for_model(&state).await;

        let mut state_lock = self.state.lock().await;
        *state_lock = state_lock.next_for_move(next_move);
    }
}
