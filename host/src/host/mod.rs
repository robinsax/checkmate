use std::collections::HashMap;

use tokio::sync::mpsc::{Receiver, Sender, channel};

use uuid::Uuid;

use crate::chess::format::StateFormat;
use crate::chess::game::Game;

enum ControlMessage {
    NewGame {id: String},
    EndGame {id: String},
    StepGame {id: String, move_alg: String}
}

pub struct HostRuntime {
    control: Receiver<ControlMessage>,
    state: Sender<StateFormat>,
    games: HashMap<String, Game>
}

impl HostRuntime {
    fn new(control: Receiver<ControlMessage>, state: Sender<StateFormat>) -> Self {
        Self {
            control, state,
            games: HashMap::new()
        }
    }

    pub async fn start(&mut self) {
        loop {
            match self.control.recv().await {
                Some(msg) => match msg {
                    ControlMessage::NewGame{id} => self.create_game(id),
                    ControlMessage::EndGame{id} => self.end_game(id),
                    ControlMessage::StepGame{id, move_alg} => self.step_game(id, move_alg)
                },
                None => { return; }
            };
        }
    }

    fn create_game(&mut self, id: String) {
        let game = Game::new();
        let state = game.get_state();

        self.games.insert(id, game);

        self.state.send(state);
    }

    fn send_game(&mut self, id: String) {
        
    }

    fn end_game(&mut self, id: String) {
        if !self.games.contains_key(&id) {
            return;
        }

        self.games.remove(&id);
    }

    fn step_game(&mut self, id: String, move_alg: String) {
        if !self.games.contains_key(&id) {
            return;
        }

        let state = self.games.get_mut(&id).unwrap().step_state(&move_alg);

        self.state.send(state);
    }
}


pub struct HostFrontend {
    state: Receiver<StateFormat>,
    control: Sender<ControlMessage>
}

impl HostFrontend {
    fn new(state: Receiver<StateFormat>, control: Sender<ControlMessage>) -> Self {
        Self {state, control}
    }

    pub async fn start_game(&self) -> String {
        let id = Uuid::new_v4().to_string();

        self.control.send(ControlMessage::NewGame{id: id.clone()});

        id
    }
}

pub fn create_host(buffer_size: usize) -> (HostRuntime, HostFrontend) {
    let (control_send, control_recv) = channel::<ControlMessage>(buffer_size);
    let (state_send, state_recv) = channel::<StateFormat>(buffer_size);

    (
        HostRuntime::new(control_recv, state_send),
        HostFrontend::new(state_recv, control_send)
    )
}
