pub mod nom_parsing;

#[cfg(test)]
mod tests;

use std::fmt::{Display, format};

use super::{Cell, Figure};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Turn {
    Move(Cell, Cell),
    PawnTransform(Cell, Figure),
    Draw,
    Reject,
    Castle(Cell)
}

#[rustfmt::skip]
impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            Turn::Move(from, to)                => format!("{from} - {to}"),
            Turn::PawnTransform(from, figure)   => format!("Pawn {from} -> {figure}"),
            Turn::Draw                          => format!("Draw"),
            Turn::Reject                        => format!("Reject"),
            Turn::Castle(to)                    => format!("Castle {to}")
        };
        write!(f, "{}", s)
    }
}

use crate::nom_extensions::parseable::{Parseable, ParseError};
impl TryFrom<&str> for Turn {
    type Error = ParseError;
    
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use nom_parsing;
        Parseable::parse(s)
    }
}
