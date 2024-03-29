mod chess_app;

use hlcd_infra;
use hlcd_infra::console_app_interface::ConsoleAppProvider;
use std::process;
use chess_app::ChessApp;

fn main() {
    // Main app component dependencies
    let console_io = hlcd_infra::get_console_io();
    let file_io = hlcd_infra::get_file_io();

    // Create main app component
    let chess_app = ChessApp::new(&console_io, &file_io);

    // Run application
    let args: Vec<String> = std::env::args().collect();
    let exit_code = ConsoleAppProvider::get(chess_app).borrow_mut().run(&args[..]);
    process::exit(exit_code);
}
