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