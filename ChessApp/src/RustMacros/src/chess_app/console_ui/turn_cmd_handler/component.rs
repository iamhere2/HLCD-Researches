use crate::chess_app::{data::Turn, rules_engine::interface::*, game_flow::interface::*};
use super::interface::*;

hlcd::define! {
    component TurnCmdHandler {
        requires {
            rules_engine: RulesEngine,
            flow_play: FlowPlay
        }

        provides { TurnCmdHandler }

        impl TurnCmdHandler {
            fn make_turn(&self, turn: Turn) -> Result<(), TurnError> {
                self.flow_play.borrow_mut().make_turn(turn)
                    .map(|_| ())
                    .map_err(|rv| TurnError::RuleViolation(rv))
            }
        }
    }
}