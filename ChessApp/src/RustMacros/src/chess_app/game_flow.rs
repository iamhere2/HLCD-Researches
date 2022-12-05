pub(super) mod interface {
    use crate::chess_app::data::{GameHistory, Color, BoardState, Turn, RuleViolation};

    hlcd::define! {
        interface GameFlowControl {
            fn game_history(&self) -> Option<&GameHistory>;
            fn player_a_color(&self) -> Option<Color>;
            fn player_b_color(&self) -> Option<Color>;
            fn next_player_color(&self) -> Option<Color>;
    
            fn new_game(&mut self, player_a_color: Color);
            fn start_from(&mut self, game_history: GameHistory, player_a_color: Color);
        } 
    
        interface ForActivePlayer {
            fn make_turn(&mut self, t: Turn) -> Result<&BoardState, RuleViolation>;
        } 
    } 
}

pub(super) mod component {
    use crate::chess_app::{data::{GameHistory, Color, Turn, BoardState, RuleViolation}, player_interface::*, rules_engine::interface::*};
    use super::interface::*;
    
    hlcd::define! {
        component ActivePassiveSyncGameFlow {
            requires {
                player_b: PassiveSyncPlayer,
                rules_engine: RulesEngine
            }
    
            state {
                game_history: Option<GameHistory> = None,
                player_a_color: Option<Color> = None,
            }
    
            provides { GameFlowControl, ForActivePlayer }
    
            impl GameFlowControl {
                fn game_history(&self) -> Option<&GameHistory> {
                    self.game_history.as_ref()
                }
            
                fn player_a_color(&self) -> Option<Color> {
                    self.player_a_color
                }
            
                fn player_b_color(&self) -> Option<Color> {
                    self.player_a_color.map(|c| !c)
                }

                fn next_player_color(&self) -> Option<Color> {
                    self.game_history.as_ref().map(|h| h.current_state().next_player_color())
                }
            
                fn new_game(&mut self, player_a_color: Color) {
                    Self::start_from(self, GameHistory::classic_initial(), player_a_color)
                }
            
                fn start_from(&mut self, game_history: GameHistory, player_a_color: Color) {
                    if game_history.is_finished() { panic!("Can't continue finished game") };
            
                    self.game_history = Some(game_history);
                    self.player_a_color = Some(player_a_color);

                    if self.next_player_color().unwrap() == !player_a_color {
                        self.make_player_b_turn();
                    }
                }
            }
            
            impl {
                fn make_player_b_turn(&mut self) {
                    let history = self.game_history.as_ref().unwrap();
                    let player_b_turn = self.player_b().turn_request(history); 
                    let new_state_b = self.rules_engine().apply(
                        history.current_state(),
                        self.player_b_color().unwrap(), 
                        player_b_turn
                    ).unwrap();
    
                    self.game_history = Some(history.with(player_b_turn, new_state_b, false));
                }
            }
    
            impl ForActivePlayer {
                fn make_turn(&mut self, player_a_turn: Turn) -> Result<&BoardState, RuleViolation> {
                    // Player A
                    let new_state_a;
                    let history;

                    {
                        history = self.game_history.as_ref().expect("Game not started");
                        let state = history.current_state();
                        new_state_a = self.rules_engine().apply(&state, self.player_a_color.unwrap(), player_a_turn)?;
                    }
            
                    self.game_history = Some(history.with(player_a_turn, new_state_a.clone(), false));

                    // PlayerB
                    self.make_player_b_turn();

                    Ok(self.game_history.as_ref().unwrap().current_state())
                }            
            }
        }
    }    
}
