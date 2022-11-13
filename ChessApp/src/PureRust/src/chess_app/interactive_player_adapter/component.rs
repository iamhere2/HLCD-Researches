use std::{cell::RefCell, rc::Rc, sync::{mpsc::{Sender, Receiver, channel}, Mutex, Arc}};

use super::super::{player_interface::{AsyncPlayerProvider, AsyncPlayerInterface}, data::{BoardState, Turn, RuleViolation}, interactive_player_adapter::interface::{InteractivePlayerAdapterInterface, InteractivePlayerAdapterAsyncProvider}};

pub struct InteractivePlayerAdapter {
    request_sender: Arc<Mutex<Sender<BoardState>>>,
    request_receiver: Arc<Mutex<Receiver<BoardState>>>,
    turn_sender: Arc<Mutex<Sender<Turn>>>,
    turn_receiver: Arc<Mutex<Receiver<Turn>>>,
    rv_sender: Arc<Mutex<Sender<RuleViolation>>>,
    rv_receiver: Arc<Mutex<Receiver<RuleViolation>>>
}

impl InteractivePlayerAdapter {
    pub fn new() -> InteractivePlayerAdapter {
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
}

impl InteractivePlayerAdapterInterface for InteractivePlayerAdapter {
    fn make_turn(&self, t: Turn) {
        self.turn_sender.lock().unwrap().send(dbg!(t));
    }
    
    fn board_state(&self) -> Option<BoardState> {
        dbg!("board state asked...");
        let mut last_bs = None;
        let rx = self.request_receiver.lock().unwrap();
        dbg!("checking for request...");
        while let Ok(bs) = rx.try_recv() {
            dbg!("request found");
            last_bs = Some(bs);
        }
        last_bs
    }
}

impl InteractivePlayerAdapterAsyncProvider for InteractivePlayerAdapter {
    fn get(it: Arc<Mutex<Self>>) -> Arc<Mutex<dyn InteractivePlayerAdapterInterface + Send + Sync>> {
        it
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