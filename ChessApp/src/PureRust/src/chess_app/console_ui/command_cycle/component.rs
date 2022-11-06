
use std::borrow::Borrow;
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;

use enum_iterator::IntoEnumIterator;

use crate::chess_app::console_ui::data::command::Command;
// Provided interfaces
use crate::hlcd_infra::console_app_interface::{ConsoleAppProvider, ConsoleAppInterface};

// Consumed interfaces
use crate::hlcd_infra::console_io_interface::*;
use super::super::board_printer::{component::*, interface::*};
use super::super::command_parser::{component::*, interface::*};
use super::super::turn_cmd_handler::{component::*, interface::*};
use super::super::game_cmd_handler::{component::*, interface::*};

// Component
// Consumes: CommandParser, TurnCmdHandler, GameCmdHandler, BoardPrinter, ConsoleIO
// Provides: ConsoleApp
// Children: -
pub struct CommandCycle {

    // Dependencies
    console_io: Rc<RefCell<dyn ConsoleIOInterface>>,
    command_parser: Rc<RefCell<dyn CommandParserInterface>>,
    turn_cmd_handler: Rc<RefCell<dyn TurnCmdHandlerInterface>>,
    game_cmd_handler: Rc<RefCell<dyn GameCmdHandlerInterface>>,
    board_printer: Rc<RefCell<dyn BoardPrinterInterface>>
}

impl ConsoleAppProvider for CommandCycle {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn ConsoleAppInterface>> { it }
}

impl CommandCycle {
    pub fn new(
        console_io: &Rc<RefCell<dyn ConsoleIOInterface>>,
        command_parser: &Rc<RefCell<dyn CommandParserInterface>>,
        turn_cmd_handler: &Rc<RefCell<dyn TurnCmdHandlerInterface>>,
        game_cmd_handler: &Rc<RefCell<dyn GameCmdHandlerInterface>>,
        board_printer: &Rc<RefCell<dyn BoardPrinterInterface>>
    ) -> Self {
        Self { 
            console_io: Rc::clone(console_io),
            command_parser: Rc::clone(command_parser),
            turn_cmd_handler: Rc::clone(turn_cmd_handler),
            game_cmd_handler: Rc::clone(game_cmd_handler),
            board_printer: Rc::clone(board_printer)
        }
    }

    fn console(&self) -> RefMut<dyn ConsoleIOInterface> {
        RefCell::borrow_mut(&self.console_io)
    }

    fn print<StrType: AsRef<str>>(&self, text_color: ConsoleColor, s: StrType) {
        let mut con = self.console();
        con.set_foreground(text_color);
        con.write(s.as_ref().to_string().as_str());
        con.set_foreground(ConsoleColor::LightGray);
    }

    fn print_hello(&self) {
        self.print(ConsoleColor::Yellow, "Hello, World! This is ChessApp, HLCD implementation with pure Rust!\n");
    }

    fn print_rainbow(&self) {
        let mut con = self.console();

        for c in ConsoleColor::into_enum_iter() {
            con.set_background(c);
            con.write("  ");
        }

        con.set_background(ConsoleColor::Black);
    }    

    fn print_error<E: std::error::Error, S:AsRef<str>>(&self, prefix: S, e: E) {
        self.print(ConsoleColor::LightRed, format!("{}: {e}\n", prefix.as_ref().to_string()));
        self.print(ConsoleColor::Gray, format!("DEBUG INFO:\n{e:?}\n"));
    }
}

impl ConsoleAppInterface for CommandCycle {
    fn run(&self) -> i32 {
        
        self.print_hello();
        self.print_rainbow();
        self.print_rainbow();
        self.print(ConsoleColor::White, "\n");

        loop {
            self.print(ConsoleColor::Yellow, "> ");
            
            let cmd_str = {
                let mut con = self.console();
                con.read_line()
            };

            let parser = RefCell::borrow(&self.command_parser);
            let pr = parser.parse(cmd_str.as_str());

            match pr {
                Ok(Command::Exit) => {
                    self.print(ConsoleColor::LightBlue, "Buy!\n");
                    return 0
                },
                Ok(Command::Help) => {
                    self.print(ConsoleColor::White, format!("{}\n", parser.get_help()))
                },
                Ok(Command::MakeTurn(t)) => {
                        let turn_cmd_handler = RefCell::borrow(&self.turn_cmd_handler);
                        if let Err(e) = turn_cmd_handler.make_turn(t) {
                            self.print_error(format!("Invalid turn {t}"), e)
                        }
                    },
                Ok(cmd) => {
                    let game_cmd_handler = RefCell::borrow(&self.game_cmd_handler);
                    if let Err(e) = game_cmd_handler.execute(cmd.clone()) {
                        self.print_error(format!("Error while executing command {}", &cmd), e)
                    }
                },
                Err(e) => self.print_error("Unknown command/turn. Enter 'help' for help", e)
            }
        }
    }
}
