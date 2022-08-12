use std::io::Error;
use super::value_types::GameHistory;

pub(super) trait StorageInterface {
    fn list_saved_games(&self) -> Result<Vec<String>, Error>;
    fn save_game(&self, gh: GameHistory, name: &str) -> Result<(), Error>;
    fn load_game(&self, name: &str) -> Result<GameHistory, Error>;
}