use super::{player_interface::*, data::{BoardState, Turn}};

hlcd::define! {
    component AiPlayer {
        provides { SyncPlayer }
        impl SyncPlayer {
            fn turn_request(&self, _bs: &BoardState) -> Turn {
                todo!()
            }
        }
    }
}
