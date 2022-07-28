use std::sync::Arc;
use std::collections::HashMap;

use log::info;
use tokio;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Sender, Receiver, channel};
use async_trait::async_trait;
use rocket::State;

use crate::chess::game::Game;
use crate::chess::agent::FormatAgent;
use crate::chess::format::StateFormat;

use crate::agents::{HeurRandAgent, HeurNoBlunderAgent};

pub type GameHostState = State<Arc<Mutex<GameHost>>>;

pub struct GameHost {
    games: HashMap<String, Arc<Game>>,
    move_txs: HashMap<String, Sender<String>>,
    state_rxs: HashMap<String, Mutex<Receiver<()>>>
}

pub struct RemoteAgent {
    move_rx: Receiver<String>
}

#[async_trait]
impl FormatAgent for RemoteAgent {
    async fn get_move_for_format(&mut self, state: &StateFormat) -> String {
        info!("remote_agent: blocking");
        let read_move = self.move_rx.recv().await.unwrap();
        info!("remote_agent: {}", read_move);

        read_move
    }
}

impl GameHost {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
            move_txs: HashMap::new(),
            state_rxs: HashMap::new()
        }
    }

    pub fn into_state(self) -> Arc<Mutex<GameHost>> {
        Arc::new(Mutex::new(self))
    }

    pub async fn get_state(&self, id: &String) -> StateFormat {
        info!("get_state: {}", id);

        self.games[id].get_state().await
    }

    async fn get_next_state(&self, id: &String) -> StateFormat {
        self.state_rxs[id].lock().await.recv().await;
        info!("next_state: {}", id);

        self.get_state(id).await
    }

    pub async fn make_move(&self, id: &String, move_alg: &String) -> Result<StateFormat, &'static str> {
        if !self.move_txs.contains_key(id) {
            return Err("invalid game id");
        }

        info!("make_move: {} <- {}", id, move_alg);
        self.move_txs[id].send(move_alg.clone()).await.unwrap();

        Ok(self.get_next_state(id).await)
    }

    pub async fn create_game(&mut self, id: &String) -> StateFormat {
        let (tx, rx) = channel(1);
        let (s_tx, s_rx) = channel(1);

        let game = Arc::new(Game::new(
            Mutex::new(Box::new(RemoteAgent{move_rx: rx})),
            Mutex::new(Box::new(HeurNoBlunderAgent::new()))
        ));
        let game2 = Arc::clone(&game);

        self.games.insert(id.to_string(), game);
        self.move_txs.insert(id.to_string(), tx);
        self.state_rxs.insert(id.to_string(), Mutex::new(s_rx));

        tokio::spawn(async move {
            let mut remote_agent = true;
            loop {
                game2.tick().await;
                if !remote_agent {
                    s_tx.send(()).await.unwrap();
                }
                remote_agent = !remote_agent;
            }
        });

        self.get_state(id).await
    }
}
