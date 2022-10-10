use std::{fmt::Display};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Color {
    Black,
    White
}

impl Color {
    fn invert(&self) -> Color {
        if self == &Color::Black { Color:: White } else { Color::Black }
    }
}

pub enum Figure {
    Knight,
    King,
    Rook,
    Bishop,
    Queen,
    Pawn
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Cell {
    v: char,
    h: u8
}

impl Cell {
    pub const V_LEFT: char = 'A';
    pub const V_RIGHT: char = 'H';
    pub const H_BOTTOM: u8 = 1;
    pub const H_TOP: u8 = 1;

    pub fn is_valid(v: char, h: u8) -> bool {
        v >= Cell::V_LEFT && v <= Cell::V_RIGHT && h >= Cell::H_BOTTOM && h <= Cell::H_TOP
    }

    pub fn new(v: char, h: u8) -> Cell {
        if !Cell::is_valid(v, h) { panic!("Invalid cell {v}{h}") };
        Cell {v, h}
    }
}

impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        let cs: Vec<_> = s.chars().collect();
        if cs.len() != 2 { panic!("Invalid cell {s}") };
        let v = cs[0];
        let h = cs[1];
        Cell::new(v, h.to_digit(10).unwrap() as u8)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}", self.v, self.h))
    }
}

pub struct GameHistory { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parce_cell() {
        let s = "A1";
        let c = Cell::from(s);
        assert_eq!(c.v, 'A');
        assert_eq!(c.h, 1);
    }
}