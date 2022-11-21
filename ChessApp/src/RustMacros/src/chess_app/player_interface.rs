use std::time::Duration;
use std::sync::mpsc::{Receiver, Sender, RecvTimeoutError};
use super::data::{BoardState, Turn, RuleViolation};

hlcd::define! {
    interface SyncPlayer {
        fn turn_request(&self, bs: &BoardState) -> Turn;
    }
}
