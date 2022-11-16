use std::time::Duration;
use std::{rc::Rc, cell::RefCell};
use std::sync::mpsc::{Receiver, Sender, RecvTimeoutError};

use super::data::{BoardState, Turn, RuleViolation};

pub trait SyncPlayerInterface {
    fn turn_request(&self, bs: &BoardState) -> Turn;
}

pub(super) trait SyncPlayerProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn SyncPlayerInterface>>; 
}
