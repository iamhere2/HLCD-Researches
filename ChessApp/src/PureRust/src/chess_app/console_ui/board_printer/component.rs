use std::{rc::Rc, cell::{RefCell, Ref}, io::{Stdout, Write}};

use crate::{hlcd_infra::console_io_interface::{ConsoleIOInterface, ConsoleColor}, chess_app::data::{Color, board, Figure}};
use crate::chess_app::data::{BoardState, Cell};

use super::interface::{BoardPrinterInterface, BoardPrinterProvider};

// Component
// Provides: BoardPrinter
// Consumes: ConsoleIO
pub struct BoardPrinter {
    console_io: Rc<RefCell<dyn ConsoleIOInterface>>
}

impl BoardPrinter {
    pub fn new(console_io: &Rc<RefCell<dyn ConsoleIOInterface>>) -> BoardPrinter {
        BoardPrinter { console_io: Rc::clone(console_io) }
    }
}

impl BoardPrinterProvider for BoardPrinter {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn BoardPrinterInterface>> {
        it
    }
}

#[rustfmt::skip]
fn figure_to_str(f: Figure) -> String {
    match f {
        Figure::Knight => "Kn".to_string(),
        Figure::King   => "Kg".to_string(),
        Figure::Rook   => "Rk".to_string(),
        Figure::Bishop => "Bs".to_string(),
        Figure::Queen  => "Qn".to_string(),
        Figure::Pawn   => "pw".to_string(),
    }
}

fn color_to_bg(c: Color) -> ConsoleColor {
    match c {
        Color::White => ConsoleColor::Brown,
        Color::Black => ConsoleColor::Red
    }
}

fn color_to_fg(c: Color) -> ConsoleColor {
    match c {
        Color::White => ConsoleColor::White,
        Color::Black => ConsoleColor::Black
    }
}

fn reset_colors(con: &mut dyn ConsoleIOInterface) {
    con.set_background(ConsoleColor::Black);
    con.set_background(ConsoleColor::LightGray);
}

impl BoardPrinterInterface for BoardPrinter {

    fn print(&self, board: &BoardState) {

        let mut con = RefCell::borrow_mut(&self.console_io);

        for h in (1..=8).rev() {

            for v in ('A'..='H') {
                
                let cell = Cell::at(v, h);
                con.set_background(color_to_bg(board::color_of(cell)));

                match board.get(cell) {
                    Some((figure, color)) => {
                        con.set_foreground(color_to_fg(color));
                        con.write(&figure_to_str(figure));
                    },
                    None => con.write("  "),
                }
            }

            con.set_background(ConsoleColor::Black);
            con.write("\n");
        }

        con.set_background(ConsoleColor::Black);
        con.write("\n");
    }
}