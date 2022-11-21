use super::data::{game_history::GameHistory, Color};

pub struct Error(pub String);

hlcd::define! {
    interface Storage {
        fn list_saved_games(&self) -> Result<Vec<String>, Error>;
        fn save_game(&self, gh: GameHistory, color: Color, name: &str) -> Result<(), Error>;
        fn load_game(&self, name: &str) -> Result<(GameHistory, Color), Error>;
    }
}
