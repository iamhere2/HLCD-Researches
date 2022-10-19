use super::Cell;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Turn {
    Move(MoveTurn)
    // , other like Castling
}

#[readonly::make]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct MoveTurn {
    from: Cell,
    to: Cell 
}