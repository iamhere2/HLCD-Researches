use std::{cell::RefCell, rc::Rc, sync::{mpsc::{Sender, Receiver, channel}, Mutex}};

use super::{player_interface::{AsyncPlayerProvider, AsyncPlayerInterface}, data::{BoardState, Turn, RuleViolation}};

struct InteractivePlayerAdapter {
    request_sender: Mutex<Sender<BoardState>>,
    request_receiver: Mutex<Receiver<BoardState>>,
    turn_sender: Mutex<Sender<Turn>>,
    turn_receiver: Mutex<Receiver<Turn>>,
    rv_sender: Mutex<Sender<RuleViolation>>,
    rv_receiver: Mutex<Receiver<RuleViolation>>
}

impl InteractivePlayerAdapter {
    fn new() -> InteractivePlayerAdapter {
        let (request_sender, request_receiver) = channel::<BoardState>();
        let (turn_sender, turn_receiver) = channel::<Turn>();
        let (rv_sender, rv_receiver) = channel::<RuleViolation>();
        InteractivePlayerAdapter { 
            request_sender: Mutex::new(request_sender), 
            request_receiver: Mutex::new(request_receiver), 
            turn_sender: Mutex::new(turn_sender), 
            turn_receiver: Mutex::new(turn_receiver),
            rv_sender: Mutex::new(rv_sender),
            rv_receiver: Mutex::new(rv_receiver)
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
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn AsyncPlayerInterface>> {
        it
    }
}

impl AsyncPlayerInterface for InteractivePlayerAdapter {
    fn next_turn_request_sender(&self) -> Mutex<Sender<BoardState>> {
        self.request_sender
    }

    fn next_turn_receiver(&self) -> Mutex<Receiver<Turn>> {
        self.turn_receiver
    }

    fn rule_violation_sender(&self) -> Mutex<Sender<RuleViolation>> {
        self.rv_sender
    }
}