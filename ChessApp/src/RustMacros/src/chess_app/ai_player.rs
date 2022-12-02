use super::{player_interface::*, data::{BoardState, Turn, Cell}};

hlcd::define! {
    component AiPlayer {
        provides { SyncPlayer }
        impl SyncPlayer {
            fn turn_request(&self, _bs: &BoardState) -> Turn {
                Turn::Move(Cell::at('E', 7), Cell::at('E', 6))
            }
        }
    }
}
