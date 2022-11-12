use super::{Turn, BoardState, Color};

#[derive(Debug)]
pub struct RuleViolation {
    board_state: BoardState,
    player_color: Color,
    turn: Turn,
    rule_violation: String
}

impl RuleViolation {
    fn new(board_state: BoardState, player_color: Color, turn: Turn, rule_violation: String) -> RuleViolation {
        RuleViolation { board_state, player_color, turn, rule_violation }
    }
}