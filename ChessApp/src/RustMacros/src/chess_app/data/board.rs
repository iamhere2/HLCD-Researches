use std::{collections::HashMap, ops::Index};

use super::{Color, Cell, Figure};

pub const V_LEFT: char = 'A';
pub const V_RIGHT: char = 'H';
pub const H_BOTTOM: u8 = 1;
pub const H_TOP: u8 = 8;

pub fn color_of(cell: Cell) -> Color {
    let hv_ndx = (cell.v as u32 - V_LEFT as u32 + cell.h as u32 - H_BOTTOM as u32) % 2;
    if hv_ndx == 0 { Color::Black } else { Color::White }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BoardState {
    figures: HashMap<Cell, (Figure, Color)>,
    next_player_color: Color,
}

impl BoardState {
    pub fn next_player_color(&self) -> Color {
        self.next_player_color
    }

    pub fn get(&self, c: Cell) -> Option<(Figure, Color)> {
        self.figures.get(&c).map(|&x| x)
    }

    pub fn with(&self, f: Figure, col: Color, c: Cell) -> Self {
        let mut figures = self.figures.clone();
        figures.insert(c, (f, col));
        BoardState { figures, next_player_color: self.next_player_color }
    }

    pub fn without(&self, c: Cell) -> Self {
        let mut figures = self.figures.clone();
        figures.remove(&c);
        BoardState { figures, next_player_color: self.next_player_color  }
    }

    pub fn with_next_player(&self, c: Color) -> Self {
        let figures = self.figures.clone();
        BoardState { figures, next_player_color: c  }
    }

    pub fn with_another_player(&self) -> Self {
        self.with_next_player(!self.next_player_color())
    }

    pub fn figures(&self) -> &HashMap<Cell, (Figure, Color)> {
        &self.figures
    }
}

pub fn empty() -> BoardState {
    BoardState { figures: HashMap::new(), next_player_color: Color::White }
}

#[rustfmt::skip]
pub fn classic_initial() -> BoardState {
    empty()
        .with(Figure::Rook,   Color::White, Cell::at('A', 1))
        .with(Figure::Knight, Color::White, Cell::at('B', 1))
        .with(Figure::Bishop, Color::White, Cell::at('C', 1))
        .with(Figure::Queen,  Color::White, Cell::at('D', 1))
        .with(Figure::King,   Color::White, Cell::at('E', 1))
        .with(Figure::Bishop, Color::White, Cell::at('F', 1))
        .with(Figure::Knight, Color::White, Cell::at('G', 1))
        .with(Figure::Rook,   Color::White, Cell::at('H', 1))
        .with(Figure::Pawn,   Color::White, Cell::at('A', 2))
        .with(Figure::Pawn,   Color::White, Cell::at('B', 2))
        .with(Figure::Pawn,   Color::White, Cell::at('C', 2))
        .with(Figure::Pawn,   Color::White, Cell::at('D', 2))
        .with(Figure::Pawn,   Color::White, Cell::at('E', 2))
        .with(Figure::Pawn,   Color::White, Cell::at('F', 2))
        .with(Figure::Pawn,   Color::White, Cell::at('G', 2))
        .with(Figure::Pawn,   Color::White, Cell::at('H', 2))
        .with(Figure::Rook,   Color::Black, Cell::at('A', 8))
        .with(Figure::Knight, Color::Black, Cell::at('B', 8))
        .with(Figure::Bishop, Color::Black, Cell::at('C', 8))
        .with(Figure::Queen,  Color::Black, Cell::at('D', 8))
        .with(Figure::King,   Color::Black, Cell::at('E', 8))
        .with(Figure::Bishop, Color::Black, Cell::at('F', 8))
        .with(Figure::Knight, Color::Black, Cell::at('G', 8))
        .with(Figure::Rook,   Color::Black, Cell::at('H', 8))
        .with(Figure::Pawn,   Color::Black, Cell::at('A', 7))
        .with(Figure::Pawn,   Color::Black, Cell::at('B', 7))
        .with(Figure::Pawn,   Color::Black, Cell::at('C', 7))
        .with(Figure::Pawn,   Color::Black, Cell::at('D', 7))
        .with(Figure::Pawn,   Color::Black, Cell::at('E', 7))
        .with(Figure::Pawn,   Color::Black, Cell::at('F', 7))
        .with(Figure::Pawn,   Color::Black, Cell::at('G', 7))
        .with(Figure::Pawn,   Color::Black, Cell::at('H', 7))
}

impl Index<Cell> for BoardState {
    type Output = (Figure, Color);
    fn index(&self, cell: Cell) -> &Self::Output {
        &self.figures[&cell]
    }
}