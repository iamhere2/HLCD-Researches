use std::{rc::Rc, cell::RefCell, sync::{Arc, Mutex, MutexGuard}};

use cancellation::{CancellationTokenSource, CancellationToken};
use lazy_static::__Deref;

use crate::chess_app::{data::{GameHistory, Color}, player_interface::{AsyncPlayerInterface}, rules_engine::interface::RulesEngineInterface};

use super::interface::{GameFlowProvider, GameFlowInterface};

pub struct GameFlow {
    player_a: Arc<Mutex<dyn AsyncPlayerInterface + Send + Sync>>,
    player_b: Arc<Mutex<dyn AsyncPlayerInterface + Send + Sync>>,
    rules_engine: Arc<Mutex<dyn RulesEngineInterface + Send + Sync>>,
    game_history: Option<GameHistory>,
    player_a_color: Option<Color>,
    cancellation_token_source: Option<CancellationTokenSource>
}

impl GameFlow {
    pub fn new(
        player_a: &Arc<Mutex<dyn AsyncPlayerInterface + Send + Sync>>,
        player_b: &Arc<Mutex<dyn AsyncPlayerInterface + Send + Sync>>,
        rules_engine: &Arc<Mutex<dyn RulesEngineInterface + Send + Sync>>
    ) -> Self {
        GameFlow {
            player_a: Arc::clone(player_a),
            player_b: Arc::clone(player_b),
            rules_engine: Arc::clone(rules_engine),
            game_history: None,
            player_a_color: None,
            cancellation_token_source: None
        }
    }

    fn turn_cycle(this: Arc<Mutex<Self>>, player_a_color: Color, cancellation_token: &Arc<CancellationToken>) {
        let mut turn_color = player_a_color;

        let this = this.lock().unwrap().deref();
        while !this.game_history.as_ref().unwrap().is_finished() && !cancellation_token.is_canceled() {
            let history = this.game_history.as_ref().unwrap();
            let state = history.states().last().unwrap();
            let player = if turn_color == player_a_color { &this.player_a } else { &this.player_b };
            let player = player.lock().unwrap();
            let player = player.deref();

            let turn = player.next_turn_sync(&state);

            if cancellation_token.is_canceled() { break };

            let rules_engine = this.rules_engine.lock().unwrap();
            let rules_engine = rules_engine.deref();

            let violation = rules_engine.check(state.clone(), turn);
            if let Err(violation) = violation {
                player.rule_violation_notification_sync(violation);
            } else {
                let next_state = rules_engine.apply(state.clone(), turn).unwrap();
                this.game_history = Some(history.with(turn, next_state, false));
                turn_color = !turn_color;
            }
        }
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

        if let Some(cts) = &self.cancellation_token_source {
            cts.cancel();
            self.cancellation_token_source = None
        }

        self.game_history = Some(game_history);
        self.player_a_color = Some(player_a_color);

        let cts = CancellationTokenSource::new();
        self.cancellation_token_source = Some(cts);

        let ct = self.cancellation_token_source.unwrap().token();

        std::thread::spawn(move || Self::turn_cycle(self, player_a_color, ct));
    }
}