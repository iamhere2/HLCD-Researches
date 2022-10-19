mod hlcd_infra;
mod chess_app;

use std::{process, rc::Rc, cell::RefCell};

use chess_app::ChessApp;
use hlcd_infra::{get_console_io, get_file_io, console_app_interface::ConsoleAppProvider};

fn main() {

    // Main app component dependencies
    let console_io = get_console_io();
    let file_io = get_file_io();

    // Create main app component
    let chess_app = Rc::new(RefCell::new(ChessApp::new(Rc::clone(&console_io), Rc::clone(&file_io))));

    // Run application
    let exit_code = ConsoleAppProvider::get(chess_app).borrow_mut().run();
    process::exit(exit_code);
}
