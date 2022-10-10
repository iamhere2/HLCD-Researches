use std::{collections::HashMap, ops::Index};

use super::{Color, Cell, Figure};

pub const V_LEFT: char = 'A';
pub const V_RIGHT: char = 'H';
pub const H_BOTTOM: u8 = 1;
pub const H_TOP: u8 = 1;

pub fn color_of(cell: Cell) -> Color {
    match (cell.v as u32 - V_LEFT as u32 + cell.h as u32 - H_BOTTOM as u32) % 2 {
        0 => Color::Black,
        1 => Color::White,
        _ => !unreachable!()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BoardState {
    figures: HashMap<Cell, (Figure, Color)>
}

impl BoardState {
    fn get(&self, c: Cell) -> Option<(Figure, Color)> {
        self.figures.get(&c).map(|&x| x)
    }

    fn with(&self, f: Figure, col: Color, c: Cell) -> Self {
        let mut figures = self.figures.clone();
        figures.insert(c, (f, col));
        BoardState { figures }
    }

    fn without(&self, c: Cell) -> Self {
        let mut figures = self.figures.clone();
        figures.remove(&c);
        BoardState { figures }
    }
}

fn empty() -> BoardState {
    BoardState { figures: HashMap::new() }
}

fn classic_initial() -> BoardState {
    empty()
        .with(Figure::Rook, Color::White, Cell::at('A', 1))
        .with(Figure::Knight, Color::White, Cell::at('B', 1))
        .with(Figure::Bishop, Color::White, Cell::at('C', 1))
        .with(Figure::Queen, Color::White, Cell::at('D', 1))
        .with(Figure::King, Color::White, Cell::at('E', 1))
        .with(Figure::Bishop, Color::White, Cell::at('F', 1))
        .with(Figure::Knight, Color::White, Cell::at('G', 1))
        .with(Figure::Rook, Color::White, Cell::at('H', 1))
        .with(Figure::Pawn, Color::White, Cell::at('A', 2))
        .with(Figure::Pawn, Color::White, Cell::at('B', 2))
        .with(Figure::Pawn, Color::White, Cell::at('C', 2))
        .with(Figure::Pawn, Color::White, Cell::at('D', 2))
        .with(Figure::Pawn, Color::White, Cell::at('E', 2))
        .with(Figure::Pawn, Color::White, Cell::at('F', 2))
        .with(Figure::Pawn, Color::White, Cell::at('G', 2))
        .with(Figure::Pawn, Color::White, Cell::at('H', 2))
        .with(Figure::Rook, Color::Black, Cell::at('A', 8))
        .with(Figure::Knight, Color::Black, Cell::at('B', 8))
        .with(Figure::Bishop, Color::Black, Cell::at('C', 8))
        .with(Figure::Queen, Color::Black, Cell::at('D', 8))
        .with(Figure::King, Color::Black, Cell::at('E', 8))
        .with(Figure::Bishop, Color::Black, Cell::at('F', 8))
        .with(Figure::Knight, Color::Black, Cell::at('G', 8))
        .with(Figure::Rook, Color::Black, Cell::at('H', 8))
        .with(Figure::Pawn, Color::Black, Cell::at('A', 7))
        .with(Figure::Pawn, Color::Black, Cell::at('B', 7))
        .with(Figure::Pawn, Color::Black, Cell::at('C', 7))
        .with(Figure::Pawn, Color::Black, Cell::at('D', 7))
        .with(Figure::Pawn, Color::Black, Cell::at('E', 7))
        .with(Figure::Pawn, Color::Black, Cell::at('F', 7))
        .with(Figure::Pawn, Color::Black, Cell::at('G', 7))
        .with(Figure::Pawn, Color::Black, Cell::at('H', 7))
}

impl Index<Cell> for BoardState {
    type Output = (Figure, Color);
    fn index(&self, cell: Cell) -> &Self::Output {
        &self.figures[&cell]
    }
}