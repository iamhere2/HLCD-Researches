use std::{rc::Rc, cell::RefCell, ops::DerefMut};

use crate::chess_app::{data::{GameHistory, Color, Turn, BoardState, RuleViolation}, player_interface::*, rules_engine::{interface::*, self}};

use super::interface::*;

pub struct GameFlow {
    player_b: SyncPlayerRef,
    rules_engine: RulesEngineRef,
    game_history: Option<GameHistory>,
    player_a_color: Option<Color>,
    next_player_color: Option<Color>
}

impl GameFlow {
    pub fn new(
        player_b: &SyncPlayerRef,
        rules_engine: &RulesEngineRef
    ) -> Self {
        GameFlow {
            player_b: Rc::clone(player_b),
            rules_engine: Rc::clone(rules_engine),
            game_history: None,
            player_a_color: None,
            next_player_color: None
        }
    }
}

impl GameFlowProvider for GameFlow {
    fn get(it: Rc<RefCell<Self>>) -> GameFlowRef {
        it
    }
}

impl FlowPlayProvider for GameFlow {
    fn get(it: Rc<RefCell<Self>>) -> FlowPlayRef {
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
        self.next_player_color = Some(player_a_color);
    }
}

impl FlowPlayInterface for GameFlow {
    fn make_turn(&mut self, t: Turn) -> Result<BoardState, RuleViolation> {
        let rules_engine = RefCell::borrow(&self.rules_engine);
        let history = self.game_history.as_ref().expect("Game not started");
        let state = history.states().last().unwrap();
        let player = self.next_player_color.unwrap();

        let new_state = rules_engine.apply(&state, player, t)?;

        self.next_player_color = Some(!player);
        self.game_history = Some(history.with(t, new_state.clone(), false));

        Ok(new_state)
    }
}