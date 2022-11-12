use std::sync::{Mutex, Arc};
use std::{rc::Rc, cell::RefCell};
use std::sync::mpsc::{Receiver, Sender};

use super::data::{BoardState, Turn, RuleViolation};

pub trait AsyncPlayerInterface {
    fn next_turn_request_sender(&self) -> Arc<Mutex<Sender<BoardState>>>;
    fn next_turn_receiver(&self) -> Arc<Mutex<Receiver<Turn>>>;
    fn rule_violation_sender(&self) -> Arc<Mutex<Sender<RuleViolation>>>;

    // Sync wrappers
    fn next_turn_sync(&self, bs: &BoardState) -> Turn {
        self.next_turn_request_sender().lock().unwrap().send(bs.clone());
        self.next_turn_receiver().lock().unwrap().recv().unwrap()
    }
    fn rule_violation_notification_sync(&self, rv: RuleViolation) {
        self.rule_violation_sender().lock().unwrap().send(rv).unwrap()
    }
}

pub(super) trait AsyncPlayerProvider {
    fn get(it: Arc<Mutex<Self>>) -> Arc<Mutex<dyn AsyncPlayerInterface + Send + Sync>>; 
}
