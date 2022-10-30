use strum::{Display, EnumString, FromRepr};


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Display, EnumString, FromRepr)]
pub enum Figure {
    Knight,
    King,
    Rook,
    Bishop,
    Queen,
    Pawn
}
