use std::collections::HashSet;
use crate::chess_app::data::{BoardState, Turn, RuleViolation, Color, GameHistory};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Condition {
    Check,
    Mate
}

hlcd::define! {
    interface RulesEngine {
        fn validate(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<(), RuleViolation>;
        fn apply(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<BoardState, RuleViolation>;
        fn get_valid_turns(&self, gh: &GameHistory) -> Vec<Turn>;
        fn get_conditions(&self, gh: &GameHistory) -> HashSet<(Condition, Color)>;
    }
}