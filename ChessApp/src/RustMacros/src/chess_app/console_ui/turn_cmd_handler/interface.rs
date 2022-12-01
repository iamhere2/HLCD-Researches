use std::{error::Error, fmt::Display};
use crate::chess_app::data::{Turn, RuleViolation};

#[derive(Clone, Debug)]
pub enum TurnError {
    RuleViolation(RuleViolation),

    #[allow(unused)]
    Other(String)
}

impl Error for TurnError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for TurnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TurnError::RuleViolation(rv) => rv.rule_violation.to_string(),
            TurnError::Other(s) => s.to_string(),
        };
        write!(f, "{s}")
    }
}

hlcd::define! {
    interface TurnCmdHandler {
        fn make_turn(&self, turn: Turn) -> Result<(), TurnError>;
    }
}

