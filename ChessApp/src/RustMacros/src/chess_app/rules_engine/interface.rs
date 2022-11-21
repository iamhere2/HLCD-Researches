use crate::chess_app::data::{BoardState, Turn, RuleViolation, Color};

hlcd::define! {

    interface RulesEngine {
        fn apply(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<BoardState, RuleViolation>;
    }
    
}