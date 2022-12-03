
use enum_iterator::IntoEnumIterator;
use crate::chess_app::console_ui::data::command::Command;
use crate::chess_app::data::Color;
use crate::chess_app::data::Turn;
// Provided interfaces
use crate::hlcd_infra::console_app_interface::*;

// Required interfaces
use crate::hlcd_infra::console_io_interface::*;
use crate::chess_app::game_flow::interface::*;
use super::super::board_printer::interface::*;
use super::super::command_parser::interface::*;
use super::super::turn_cmd_handler::interface::*;
use super::super::game_cmd_handler::interface::*;

hlcd::define! {
    component CommandCycle {
        provides { ConsoleApp }
        
        requires {
            console: ConsoleIO,
            parser: CommandParser, 
            turn_cmd_handler: TurnCmdHandler, 
            game_cmd_handler: GameCmdHandler, 
            printer: BoardPrinter,
            flow: GameFlowControl,
        }

        impl {
            fn print<S: AsRef<str>>(&self, text_color: ConsoleColor, s: S) {
                let mut con = self.console_mut();
                con.set_foreground(text_color);
                con.write(s.as_ref().to_string().as_str());
                con.set_foreground(ConsoleColor::LightGray);
            }
        
            fn print_hello(&self) {
                self.print(ConsoleColor::Yellow, "Hello, World! This is ChessApp, HLCD implementation with pure Rust!\n");
            }
        
            fn print_rainbow(&self) {
                let mut con = self.console_mut();
        
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

            fn print_turn(&self, c: Color, t: Turn) {
                self.print(ConsoleColor::White, format!("{}:{:?}\n", c, t));
            }
        }

        impl ConsoleApp {
            fn run(&self) -> i32 {
                
                self.print_hello();
                self.print_rainbow();
                self.print_rainbow();
                self.print(ConsoleColor::White, "\n");

                loop {
                    {
                        let flow = self.flow();
                        let board = flow.game_history().map(|h| h.states().last().unwrap());

                        if let Some(board) = board {
                            let printer = self.printer();
                            printer.print(board);
                            self.print(ConsoleColor::Yellow, format!("{:?} > ", flow.player_a_color().unwrap()));
                        } else {
                            self.print(ConsoleColor::Yellow, "cmd > ");
                        }
                    }
                    
                    let cmd_str = {
                        let con = self.console();
                        con.read_line()
                    };

                    let parser = self.parser();
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
                                self.print_turn(self.flow().next_player_color().unwrap(), t);

                                let turn_cmd_handler = self.turn_cmd_handler();
                                if let Err(e) = turn_cmd_handler.make_turn(t) {
                                    self.print_error(format!("Invalid turn {t}"), e)
                                };

                                self.print_turn(
                                    !self.flow().next_player_color().unwrap(), 
                                    *self.flow().game_history().unwrap().turns().last().unwrap());
                            },
                        Ok(cmd) => {
                            let game_cmd_handler = self.game_cmd_handler();
                            if let Err(e) = game_cmd_handler.execute(cmd.clone()) {
                                self.print_error(format!("Error while executing command {}", &cmd), e)
                            }
                        },
                        Err(e) => self.print_error("Unknown command/turn. Enter 'help' for help", e)
                    }
                }
            }
        }
    }
}