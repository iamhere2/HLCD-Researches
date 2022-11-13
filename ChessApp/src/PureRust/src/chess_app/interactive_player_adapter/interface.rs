use std::sync::{Arc, Mutex};

use crate::chess_app::data::{Turn, BoardState};

pub trait InteractivePlayerAdapterInterface {
    fn make_turn(&self, t: Turn);
    fn board_state(&self) -> Option<BoardState>;
}

pub trait InteractivePlayerAdapterAsyncProvider {
    fn get(it: Arc<Mutex<Self>>) -> Arc<Mutex<dyn InteractivePlayerAdapterInterface + Send + Sync>>; 
}


