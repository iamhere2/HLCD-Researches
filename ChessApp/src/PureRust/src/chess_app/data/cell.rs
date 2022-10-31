pub mod nom_parsing;

#[cfg(test)]
mod tests;

use std::fmt::Display;
use super::board;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[readonly::make]
pub struct Cell {
    pub v: char,
    pub h: u8
}

impl Cell {
    pub fn is_valid(v: char, h: u8) -> bool {
        v >= board::V_LEFT && v <= board::V_RIGHT && h >= board::H_BOTTOM && h <= board::H_TOP
    }

    pub fn at(v: char, h: u8) -> Cell {
        if !Cell::is_valid(v, h) { panic!("Invalid V or H {v}{h}") };
        Cell {v, h}
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.v, self.h)
    }
}


use crate::nom_extensions::parseable::{Parseable, ParseError};
impl<'a> TryFrom<&'a str> for Cell {
    type Error = ParseError;
    
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        use nom_parsing;
        Parseable::parse(s)
    }
}

