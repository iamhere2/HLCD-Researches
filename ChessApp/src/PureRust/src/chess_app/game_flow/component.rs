use std::{rc::Rc, cell::RefCell, ops::DerefMut};

use crate::chess_app::{data::{GameHistory, Color, Turn, BoardState}, player_interface::*, rules_engine::interface::*};

use super::interface::*;

pub struct GameFlow {
    player_b: Rc<RefCell<dyn SyncPlayerInterface>>,
    rules_engine: Rc<RefCell<dyn RulesEngineInterface>>,
    game_history: Option<GameHistory>,
    player_a_color: Option<Color>,
}

impl GameFlow {
    pub fn new(
        player_b: &Rc<RefCell<dyn SyncPlayerInterface>>,
        rules_engine: &Rc<RefCell<dyn RulesEngineInterface>>
    ) -> Self {
        GameFlow {
            player_b: Rc::clone(player_b),
            rules_engine: Rc::clone(rules_engine),
            game_history: None,
            player_a_color: None
        }
    }
}

impl GameFlowProvider for GameFlow {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn GameFlowInterface>> {
        it
    }
}

impl FlowPlayProvider for GameFlow {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn FlowPlayInterface>> {
        it
    }
}

impl GameFlowInterface for GameFlow {
    fn game_history(&self) -> Option<&GameHistory> {
        self.game_history.as_ref()
    }

    fn player_a_color(&self) -> Option<Color> {
        self.player_a_color
    }

    fn player_b_color(&self) -> Option<Color> {
        self.player_a_color.map(|c| !c)
    }

    fn new_game(&mut self, player_a_color: Color) {
        Self::start_from(self, GameHistory::classic_initial(), player_a_color)
    }

    fn start_from(&mut self, game_history: GameHistory, player_a_color: Color) {
        if game_history.is_finished() { panic!("Can't continue finished game") };

        dbg!("New game creating...");
        self.game_history = Some(game_history);
        self.player_a_color = Some(player_a_color);
    }
}

impl FlowPlayInterface for GameFlow {
    fn make_turn(&self, t: Turn) -> BoardState {
        todo!()
    }
}