use std::ops::Not;

use strum::{EnumString, Display};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, EnumString, Display)]
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert() {
        assert_eq!(!Color::Black, Color::White);
        assert_eq!(!Color::White, Color::Black);
    }
}