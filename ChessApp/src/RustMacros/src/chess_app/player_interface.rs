use super::data::{BoardState, Turn};

hlcd::define! {
    interface SyncPlayer {
        fn turn_request(&self, bs: &BoardState) -> Turn;
    }
}
