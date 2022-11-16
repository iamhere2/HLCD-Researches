use std::{rc::Rc, cell::RefCell};

use crate::chess_app::data::{GameHistory, Color, BoardState, Turn};

pub trait GameFlowInterface {
    fn game_history(&self) -> Option<&GameHistory>;
    fn player_a_color(&self) -> Option<Color>;
    fn player_b_color(&self) -> Option<Color>;

    fn new_game(&mut self, player_a_color: Color);
    fn start_from(&mut self, game_history: GameHistory, player_a_color: Color);
} 

pub trait GameFlowProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn GameFlowInterface>>;
} 

pub trait FlowPlayInterface {
    fn make_turn(&self, t: Turn) -> BoardState;
} 

pub trait FlowPlayProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn FlowPlayInterface>>;
} 