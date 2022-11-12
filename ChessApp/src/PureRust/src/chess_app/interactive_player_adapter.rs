use std::{cell::RefCell, rc::Rc, sync::{mpsc::{Sender, Receiver, channel}, Mutex, Arc}};

use super::{player_interface::{AsyncPlayerProvider, AsyncPlayerInterface}, data::{BoardState, Turn, RuleViolation}};

pub(super) struct InteractivePlayerAdapter {
    request_sender: Arc<Mutex<Sender<BoardState>>>,
    request_receiver: Arc<Mutex<Receiver<BoardState>>>,
    turn_sender: Arc<Mutex<Sender<Turn>>>,
    turn_receiver: Arc<Mutex<Receiver<Turn>>>,
    rv_sender: Arc<Mutex<Sender<RuleViolation>>>,
    rv_receiver: Arc<Mutex<Receiver<RuleViolation>>>
}

impl InteractivePlayerAdapter {
    pub(super) fn new() -> InteractivePlayerAdapter {
        let (request_sender, request_receiver) = channel::<BoardState>();
        let (turn_sender, turn_receiver) = channel::<Turn>();
        let (rv_sender, rv_receiver) = channel::<RuleViolation>();
        InteractivePlayerAdapter { 
            request_sender: Arc::new(Mutex::new(request_sender)), 
            request_receiver: Arc::new(Mutex::new(request_receiver)), 
            turn_sender: Arc::new(Mutex::new(turn_sender)), 
            turn_receiver: Arc::new(Mutex::new(turn_receiver)),
            rv_sender: Arc::new(Mutex::new(rv_sender)),
            rv_receiver: Arc::new(Mutex::new(rv_receiver))
        }
    }

    fn make_turn(&self, t: Turn) {
        self.turn_sender.lock().unwrap().send(t);
    }

    fn board_state(&self) -> Option<BoardState> {
        let mut last_bs = None;
        let rx = self.request_receiver.lock().unwrap();
        while let Ok(bs) = rx.try_recv() {
            last_bs = Some(bs);
        }
        last_bs
    }
}

impl AsyncPlayerProvider for InteractivePlayerAdapter {
    fn get(it: Arc<Mutex<Self>>) -> Arc<Mutex<dyn AsyncPlayerInterface + Send + Sync>> {
        it
    }
}

impl AsyncPlayerInterface for InteractivePlayerAdapter {
    fn next_turn_request_sender(&self) -> Arc<Mutex<Sender<BoardState>>> {
        Arc::clone(&self.request_sender)
    }

    fn next_turn_receiver(&self) -> Arc<Mutex<Receiver<Turn>>> {
        Arc::clone(&self.turn_receiver)
    }

    fn rule_violation_sender(&self) -> Arc<Mutex<Sender<RuleViolation>>> {
        Arc::clone(&self.rv_sender)
    }
}