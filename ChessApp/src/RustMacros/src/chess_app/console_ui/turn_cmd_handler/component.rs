use crate::chess_app::{data::Turn, rules_engine::interface::*, game_flow::interface::*};
use super::interface::*;

hlcd::define! {
    component TurnCmdHandler {
        requires {
            rules_engine: RulesEngine,
            flow: ForActivePlayer
        }

        provides { TurnCmdHandler }

        impl TurnCmdHandler {
            fn make_turn(&self, turn: Turn) -> Result<(), TurnError> {
                self.flow_mut().make_turn(turn)
                    .map(|_| ())
                    .map_err(|rv| TurnError::RuleViolation(rv))
            }
        }
    }
}