use super::data::{BoardState, Turn};

hlcd::define! {
    interface PassiveSyncPlayer {
        fn turn_request(&self, bs: &BoardState) -> Turn;
    }
}
