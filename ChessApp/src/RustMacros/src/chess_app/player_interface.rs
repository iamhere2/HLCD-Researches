use super::data::{Turn, GameHistory};

hlcd::define! {
    interface PassiveSyncPlayer {
        fn turn_request(&self, gh: &GameHistory) -> Turn;
    }
}
