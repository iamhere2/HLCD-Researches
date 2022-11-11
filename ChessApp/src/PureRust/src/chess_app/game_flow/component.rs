use std::{rc::Rc, cell::RefCell, sync::Arc};

use cancellation::{CancellationTokenSource, CancellationToken};

use crate::chess_app::{data::{GameHistory, Color}, player_interface::PlayerInterface, rules_engine::interface::RulesEngineInterface};

use super::interface::{GameFlowProvider, GameFlowInterface};

pub struct GameFlow {
    player_a: Rc<RefCell<dyn PlayerInterface>>,
    player_b: Rc<RefCell<dyn PlayerInterface>>,
    rules_engine: Rc<RefCell<dyn RulesEngineInterface>>,
    game_history: Option<GameHistory>,
    player_a_color: Option<Color>,
    cancellation_token_source: Option<CancellationTokenSource>
}

impl GameFlow {
    pub fn new(
        player_a: &Rc<RefCell<dyn PlayerInterface>>,
        player_b: &Rc<RefCell<dyn PlayerInterface>>,
        rules_engine: &Rc<RefCell<dyn RulesEngineInterface>>
    ) -> Self {
        GameFlow {
            player_a: Rc::clone(player_a),
            player_b: Rc::clone(player_b),
            rules_engine: Rc::clone(rules_engine),
            game_history: None,
            player_a_color: None,
            cancellation_token_source: None
        }
    }

    fn turn_cycle(&mut self, player_a_color: Color, cancellation_token: &Arc<CancellationToken>) {

    }
}

impl GameFlowProvider for GameFlow {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn GameFlowInterface>> {
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

        if let Some(cts) = self.cancellation_token_source {
            cts.cancel();
            self.cancellation_token_source = None
        }

        self.game_history = Some(game_history);
        self.player_a_color = Some(player_a_color);

        let cts = CancellationTokenSource::new();
        self.cancellation_token_source = Some(cts);

        let ct = cts.token();

        std::thread::spawn(|| self.turn_cycle(player_a_color, ct));
    }
}