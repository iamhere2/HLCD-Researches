use std::sync::{Mutex, Arc};
use std::time::Duration;
use std::{rc::Rc, cell::RefCell};
use std::sync::mpsc::{Receiver, Sender, RecvTimeoutError};

use super::data::{BoardState, Turn, RuleViolation};

pub trait AsyncPlayerInterface {
    fn next_turn_request_sender(&self) -> Arc<Mutex<Sender<BoardState>>>;
    fn next_turn_receiver(&self) -> Arc<Mutex<Receiver<Turn>>>;
    fn rule_violation_sender(&self) -> Arc<Mutex<Sender<RuleViolation>>>;

    // Sync wrappers
    fn next_turn_sync(&self, bs: &BoardState) -> Result<Turn, RecvTimeoutError> {
        dbg!("New turn request!");
        {
            self.next_turn_request_sender().lock().unwrap().send(bs.clone());
        }
        dbg!("...awaiting for response...");
        {
            let x = self.next_turn_receiver();
            let y = x.lock();
            let z = y.unwrap();
            z.recv_timeout(Duration::from_secs(10))
        }
    }
    fn rule_violation_notification_sync(&self, rv: RuleViolation) {
        self.rule_violation_sender().lock().unwrap().send(rv).unwrap()
    }
}

pub(super) trait AsyncPlayerProvider {
    fn get(it: Arc<Mutex<Self>>) -> Arc<Mutex<dyn AsyncPlayerInterface + Send + Sync>>; 
}
