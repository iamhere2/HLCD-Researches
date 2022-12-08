use hlcd_infra::console_io_interface::*;
use crate::chess_app::data::{Color, board, Piece, BoardState, Cell};
use super::interface::*;

hlcd::define! {
    component BoardPrinter {
        requires { console_io: ConsoleIO }
        provides { BoardPrinter }
        impl BoardPrinter {
            fn print(&self, board: &BoardState) {

                let mut con = self.console_io_mut();
        
                for h in (1..=8).rev() {

                    con.set_foreground(ConsoleColor::Cyan);
                    con.write(format!("{h} ").as_str());
        
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
                con.set_foreground(ConsoleColor::Cyan);
                con.write("  A B C D E F G H\n");

                con.set_foreground(ConsoleColor::White);
                con.write(format!("Next player: {}\n", board.next_player_color()).as_str());
            }
        }
    }
}

#[rustfmt::skip]
fn figure_to_str(f: Piece) -> String {
    match f {
        Piece::Knight => "Kn".to_string(),
        Piece::King   => "Kg".to_string(),
        Piece::Rook   => "Rk".to_string(),
        Piece::Bishop => "Bs".to_string(),
        Piece::Queen  => "Qn".to_string(),
        Piece::Pawn   => "pw".to_string(),
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
