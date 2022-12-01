use hlcd_infra::console_io_interface::*;
use crate::chess_app::data::{Color, board, Figure, BoardState, Cell};
use super::interface::*;

hlcd::define! {
    component BoardPrinter {
        requires { console_io: ConsoleIO }
        provides { BoardPrinter }
        impl BoardPrinter {
            fn print(&self, board: &BoardState) {

                let mut con = self.console_io_mut();
        
                for h in (1..=8).rev() {
        
                    for v in 'A'..='H' {
                        
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
