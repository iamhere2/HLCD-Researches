use super::{player_interface::*, data::{Turn, GameHistory}, rules_engine::interface::*};

hlcd::define! {
    component StupidAiPlayer {
        requires { RulesEngine }
        provides { PassiveSyncPlayer }
        impl PassiveSyncPlayer {
            fn turn_request(&self, gh: &GameHistory) -> Turn {
                *self.rules_engine().get_valid_turns(gh).first().unwrap()
            }
        }
    }
}
