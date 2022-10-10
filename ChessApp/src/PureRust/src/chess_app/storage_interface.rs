use std::{io::Error, rc::Rc, cell::RefCell};
use super::data::game_history::GameHistory;

pub(super) trait StorageInterface {
    fn list_saved_games(&self) -> Result<Vec<String>, Error>;
    fn save_game(&self, gh: GameHistory, name: &str) -> Result<(), Error>;
    fn load_game(&self, name: &str) -> Result<GameHistory, Error>;
}

pub(super) trait StorageProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn StorageInterface>>; 
}
