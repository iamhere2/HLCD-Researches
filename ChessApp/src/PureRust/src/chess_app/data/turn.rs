use super::Cell;

pub enum Turn {
    Move(MoveTurn)
    // , other like Castling
}

#[readonly::make]
pub struct MoveTurn {
    from: Cell,
    to: Cell 
}