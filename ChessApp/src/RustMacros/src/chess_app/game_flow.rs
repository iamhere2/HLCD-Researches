pub(super) mod interface {
    use crate::chess_app::data::{GameHistory, Color, BoardState, Turn, RuleViolation};

    hlcd::define! {
        interface GameFlow {
            fn game_history(&self) -> Option<&GameHistory>;
            fn player_a_color(&self) -> Option<Color>;
            fn player_b_color(&self) -> Option<Color>;
    
            fn new_game(&mut self, player_a_color: Color);
            fn start_from(&mut self, game_history: GameHistory, player_a_color: Color);
        } 
    
        interface FlowPlay {
            fn make_turn(&mut self, t: Turn) -> Result<BoardState, RuleViolation>;
        } 
    } 
}

pub(super) mod component {
    use crate::chess_app::{data::{GameHistory, Color, Turn, BoardState, RuleViolation}, player_interface::*, rules_engine::interface::*};
    use super::interface::*;
    
    hlcd::define! {
        component GameFlow {
            requires {
                player_b: SyncPlayer,
                rules_engine: RulesEngine
            }
    
            state {
                game_history: Option<GameHistory> = None,
                player_a_color: Option<Color> = None,
                next_player_color: Option<Color> = None
            }
    
            provides { GameFlow, FlowPlay }
    
            impl GameFlow {
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
            
                    self.game_history = Some(game_history);
                    self.player_a_color = Some(player_a_color);
                    self.next_player_color = Some(player_a_color);
                }
            }
    
            impl FlowPlay {
                fn make_turn(&mut self, t: Turn) -> Result<BoardState, RuleViolation> {
                    let player;
                    let new_state;
                    let history;
                    {
                        let rules_engine = self.rules_engine();
                        history = self.game_history.as_ref().expect("Game not started");
                        let state = history.states().last().unwrap();
                        player = self.next_player_color.unwrap();
                        new_state = rules_engine.apply(&state, player, t)?;
                    }
            
                    self.next_player_color = Some(!player);
                    self.game_history = Some(history.with(t, new_state.clone(), false));
            
                    Ok(new_state)
                }            
            }
        }
    }    
}
