mod hlcd_infra;
mod chess_app;

fn main() {
    hlcd_infra::console_app::run::<chess_app::ChessApp>()
}
