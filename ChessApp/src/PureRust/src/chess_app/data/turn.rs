use super::{Cell, Figure};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Turn {
    Move(Cell, Cell),
    PawnTransform(Cell, Figure),
    Draw,
    Reject,
    Castle(Cell)
}