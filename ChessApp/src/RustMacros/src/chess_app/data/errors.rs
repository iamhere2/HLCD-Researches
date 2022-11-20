use super::{Turn, BoardState, Color};

#[derive(Debug, Clone)]
#[readonly::make]
pub struct RuleViolation {
    pub board_state: BoardState,
    pub player_color: Color,
    pub turn: Turn,
    pub rule_violation: String
}

impl RuleViolation {
    pub fn new(board_state: BoardState, player_color: Color, turn: Turn, rule_violation: String) -> RuleViolation {
        RuleViolation { board_state, player_color, turn, rule_violation }
    }
}
