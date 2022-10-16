use std::io::stdin;

use colored::{Colorize, Color};
use super::console_io_interface::*;

pub(super) struct Console {
    foreground_color: ConsoleColor,
    background_color: ConsoleColor,
}

impl Console {
    pub fn new() -> Console {
        Console { 
            foreground_color: ConsoleColor::LightGray,
            background_color: ConsoleColor::Black
        }
    }
}

impl From<ConsoleColor> for Color {
    fn from(cc: ConsoleColor) -> Self {
        match cc {
            ConsoleColor::Black        => Color::Black,
            ConsoleColor::Blue         => Color::Blue,
            ConsoleColor::Green        => Color::Green,
            ConsoleColor::Cyan         => Color::Cyan,
            ConsoleColor::Red          => Color::Red,
            ConsoleColor::Magenta      => Color::Magenta,
            ConsoleColor::Brown        => Color::Yellow,
            ConsoleColor::Gray         => Color::White,
            ConsoleColor::LightGray    => Color::BrightBlack,
            ConsoleColor::LightBlue    => Color::BrightBlue,
            ConsoleColor::LightGreen   => Color::BrightGreen,
            ConsoleColor::LightCyan    => Color::BrightCyan,
            ConsoleColor::LightRed     => Color::BrightRed,
            ConsoleColor::LightMagenta => Color::BrightMagenta,
            ConsoleColor::Yellow       => Color::BrightYellow,
            ConsoleColor::White        => Color::BrightWhite,
        }
    }
}


impl ConsoleIOInterface for Console {
    fn set_foreground(&mut self, color: ConsoleColor) {
        self.foreground_color = color
    }

    fn set_background(&mut self, color: ConsoleColor) {
        self.background_color = color
    }

    fn write(&self, s: &str) {
        let bg = Color::from(self.background_color);
        let fg = Color::from(self.foreground_color);
        print!("{}", s.on_color(bg).color(fg))
    }

    fn read_line(&self) -> String {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s
    }
}
