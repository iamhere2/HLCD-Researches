// Children modules
mod data;
mod board_printer;
mod board_printer_interface;
mod command_cycle;
mod command_parser;
mod command_parser_interfaces;
mod game_cmd_handler;
mod turn_cmd_handler;

use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::{ Ref, RefCell, RefMut };
use std::io::Write;

// For provided interfaces
use crate::hlcd_infra::console_app_interface::*;

// For consumed interfaces
use crate::hlcd_infra::console_io_interface::*;
use board_printer::BoardPrinter;
use board_printer_interface::BoardPrinterInterface;
use enum_iterator::IntoEnumIterator;

use super::data::board;
use super::storage_interface::*;

// Component
// Provides: ConsoleApp
// Consumes: ConsoleUI, Storage
pub(super) struct ConsoleUI {

    // Owned dependencies
    console_io: Rc<RefCell<dyn ConsoleIOInterface>>,
    storage: Rc<RefCell<dyn StorageInterface>>,

    // Children components 
    board_printer: Rc<RefCell<dyn BoardPrinterInterface>>
}

impl ConsoleUI {
    // Constructor with dependencies
    pub(super) fn new(
        console_io: Rc<RefCell<dyn ConsoleIOInterface>>,
        storage: Rc<RefCell<dyn StorageInterface>>) 
    -> ConsoleUI {
        let console_io = Rc::clone(&console_io); 
        let storage = Rc::clone(&storage);
        let board_printer = Rc::new(RefCell::new(BoardPrinter::new(&console_io))); 
        ConsoleUI { console_io, storage, board_printer }
    }
        
    fn print_hello(&self) {
        let mut con = RefCell::borrow_mut(&self.console_io);
        con.set_background(ConsoleColor::Black);
        con.set_foreground(ConsoleColor::Yellow);
        
        con.write("Hello, World! This is ChessApp, HLCD implementation with pure Rust!\n\n");
    }

    fn print_rainbow(&self) {
        let mut con = RefCell::borrow_mut(&self.console_io);

        for c in ConsoleColor::into_enum_iter() {
            con.set_background(c);
            con.write("  ");
        }

        con.set_background(ConsoleColor::Black);
        con.write("\n");
    }

    fn print_board(&self) {
        let b = board::classic_initial();
        let printer = self.board_printer.borrow();
        printer.print(b);
    }
}

// Provided interface - implemented by itself
impl ConsoleAppProvider for ConsoleUI {
    fn get(it: Rc<RefCell<ConsoleUI>>) -> Rc<RefCell<dyn ConsoleAppInterface>> { it }
}

impl ConsoleAppInterface for ConsoleUI {
    fn run(&self) -> i32 {
        self.print_rainbow();
        self.print_hello();
        self.print_board();
        0
    }
}
