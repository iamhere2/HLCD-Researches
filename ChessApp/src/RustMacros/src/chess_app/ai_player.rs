use super::{player_interface::*, data::{BoardState, Turn, RuleViolation}};

hlcd::define! {
    component AiPlayer {
        provides { SyncPlayer }
        impl SyncPlayer {
            fn turn_request(&self, bs: &BoardState) -> Turn {
                todo!()
            }
        }
    }
}
