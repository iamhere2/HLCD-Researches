mod hlcd_infra;
mod chess_app;

use std::process;

use chess_app::ChessApp;
use hlcd_infra::{get_console_io, get_file_io};

fn main() {

    // Create main app component
    let mut chess_app = ChessApp::new();

    // Assign infrastructure dependencies
    let console_io = get_console_io();
    chess_app.set_console_io(console_io);

    let file_io = get_file_io();
    chess_app.set_file_io(file_io);

    // Run application
    let exit_code = chess_app.get_console_app().run();

    process::exit(exit_code);
}
