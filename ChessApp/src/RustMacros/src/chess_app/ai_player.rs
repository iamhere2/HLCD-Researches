use super::{player_interface::*, data::{Turn, GameHistory}, rules_engine::interface::*};
use rand::prelude::*;

hlcd::define! {
    component StupidAiPlayer {
        requires { RulesEngine }
        provides { PassiveSyncPlayer }
        impl PassiveSyncPlayer {
            fn turn_request(&self, gh: &GameHistory) -> Turn {
                let valid_turns = self.rules_engine().get_valid_turns(gh);
                let mut rng = thread_rng();
                let rnd_ndx = rng.gen_range(0..valid_turns.len());
                *valid_turns.get(rnd_ndx).unwrap()
            }
        }
    }
}
