use crate::chess_app::data::{BoardState, Turn, RuleViolation, Color, GameHistory};

hlcd::define! {
    interface RulesEngine {
        fn validate(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<(), RuleViolation>;
        fn apply(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<BoardState, RuleViolation>;
        fn get_valid_turns(&self, gh: &GameHistory) -> Vec<Turn>;
    }
}