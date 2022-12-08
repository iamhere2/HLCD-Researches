pub mod nom_parsing;

#[cfg(test)]
mod tests;

use strum::{Display, EnumString, FromRepr};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Display, EnumString, FromRepr)]
pub enum Piece {
    Knight,
    King,
    Rook,
    Bishop,
    Queen, 
    Pawn
}
