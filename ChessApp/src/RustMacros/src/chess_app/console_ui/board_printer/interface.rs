use crate::chess_app::data::board::BoardState;

hlcd::define! {
    interface BoardPrinter {
        fn print(&self, board: &BoardState);
    }
}
