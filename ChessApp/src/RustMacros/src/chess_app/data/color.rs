pub mod nom_parsing;

#[cfg(test)]
mod tests;

use std::ops::Not;
use strum::{EnumString, Display, EnumIter};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, EnumString, Display, EnumIter)]
pub enum Color {
    Black,
    White
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black
        }
    }
}
