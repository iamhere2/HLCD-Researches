pub mod nom_parsing;

#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter};
use super::board;

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd)]
#[readonly::make]
pub struct Cell {
    pub v: char,
    pub h: u8
}

impl Cell {
    pub fn is_valid(v: char, h: u8) -> bool {
        use board::*;
        v >= V_LEFT && v <= V_RIGHT && h >= H_BOTTOM && h <= H_TOP
    }

    pub fn at(v: char, h: u8) -> Cell {
        if !Cell::is_valid(v, h) { panic!("Invalid V or H {v}{h}") };
        Cell {v, h}
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.v, self.h)
    }
}


use nom_extensions::parseable::{Parseable, ParseError};
impl TryFrom<&str> for Cell {
    type Error = ParseError;
    
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Parseable::parse(s)
    }
}

