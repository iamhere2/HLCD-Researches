use std::io::Error;
use crate::chess_app::data::board::BoardState;
use super::super::data::*;

hlcd::define! {
    interface BoardPrinter {
        fn print(&self, board: &BoardState);
    }
}
