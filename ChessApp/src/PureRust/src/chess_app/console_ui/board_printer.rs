use std::{rc::Rc, cell::{RefCell, Ref}, io::{Stdout, Write}};

use crate::{hlcd_infra::console_io_interface::{ConsoleIOInterface, ConsoleColor}};
use crate::chess_app::data::{BoardState, Cell};

use super::board_printer_interface::BoardPrinterInterface;

// Component
// Provides: BoardPrinter
// Consumes: ConsoleIO
pub(super) struct BoardPrinter {
    console_io: Rc<RefCell<dyn ConsoleIOInterface>>
}

impl BoardPrinter {
    pub(super) fn new(console_io: &Rc<RefCell<dyn ConsoleIOInterface>>) -> BoardPrinter {
        BoardPrinter { console_io: Rc::clone(console_io) }
    }

    fn write(&self, bg: ConsoleColor, fg: ConsoleColor, s: &str) {
        let mut c = self.console_io.borrow_mut();
        c.set_background(bg);
        c.set_foreground(fg);
        c.write(s)
    }
}

impl BoardPrinterInterface for BoardPrinter {

    fn print(&self, board: BoardState) {

        self.write(ConsoleColor::Gray, ConsoleColor::Blue, format!("test: {:?}", board[Cell::at('E', 2)]).as_str());
    }
}