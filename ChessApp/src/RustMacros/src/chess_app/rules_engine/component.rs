use std::ops::{Add, Sub};

use crate::chess_app::data::{Color, Cell, Turn, RuleViolation, BoardState, Figure, GameHistory, board};
use super::interface::*;

type Shift = (i8, i8);
type Path = Vec<Cell>;

hlcd::define! {

     component RulesEngine {

        provides { RulesEngine }

        impl RulesEngine {
            fn apply(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<BoardState, RuleViolation> {
                match turn {
                    Turn::Move(from, to) => Self::apply_move(bs, player, from, to),
                    Turn::PawnTransform(_, _) => unimplemented!(),
                    Turn::Draw => todo!(),
                    Turn::Reject => todo!(),
                    Turn::Castle(_) => todo!(),
                }.map_err(|e| RuleViolation::new(bs.clone(), player, turn, e))
            }

            fn validate(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<(), RuleViolation> {
                self.apply(bs, player, turn).map(|_| ())
            }

            fn get_valid_turns(&self, gh: &GameHistory) -> Vec<Turn> {
                let board = gh.states().last().unwrap();
                let player = board.next_player_color();
                let mut turns = vec![];
                let from_cells = board.figures().iter()
                    .filter(|(_, (_, c))| *c == player)
                    .map(|(cell, _)| *cell)
                    .collect::<Vec<_>>();

                for from in from_cells {
                    let to_cells = self.get_figure_valid_moves(board, from);
                    turns.extend(to_cells.iter().map(|to| Turn::Move(from, *to)));
                }

                turns
            }
        }

        impl {
            fn apply_move(board: &BoardState, player: Color, from: Cell, to: Cell) -> Result<BoardState, String> {
                if board.next_player_color() != player {
                    return Err("It's another player's turn now".to_string()) 
                };

                let (figure, color) = board.get(from)
                    .ok_or("No figure at source cell")?;

                if color != player { 
                    return Err("Can't move figures of other color".to_string()) 
                };

                if from == to { 
                    return Err("Can't move to the same cell".to_string()) 
                };

                let (to_figure, to_color) = match board.get(to) {
                    Some((f, c)) => (Some(f), Some(c)),
                    None => (None, None)
                };

                if to_color.is_some() && to_color.unwrap() == color { 
                    return Err("Can't eat your own color".to_string()) 
                };

                let (dv, dh) = to - from;

                let can_move_that_way = 
                    match figure {
                        Figure::King => dh == 0 && dv.abs() == 1 || dh.abs() == 1 && dv == 0,
                        Figure::Rook => dh == 0 || dv == 0,
                        Figure::Queen => dh == 0 || dv == 0 || dh.abs() == dv.abs(),
                        Figure::Bishop => dh.abs() == dv.abs(),
                        Figure::Pawn => 
                               (color == Color::White && dh ==  1 && dv == 0 && to_figure.is_none())
                            || (color == Color::Black && dh == -1 && dv == 0 && to_figure.is_none())
                            || (color == Color::White && from.h == 2 && dh ==  2 && dv == 0 && to_figure.is_none())
                            || (color == Color::Black && from.h == 7 && dh == -2 && dv == 0 && to_figure.is_none())
                            || (color == Color::White && dh.abs() == 1 && dv ==  1 && to_figure.is_some())
                            || (color == Color::Black && dh.abs() == 1 && dv == -1 && to_figure.is_some()),
                        Figure::Knight => 
                               dh.abs() == 2 && dv.abs() == 1
                            || dh.abs() == 1 && dv.abs() == 2
                    };

                if !dbg!(can_move_that_way) { 
                    return Err(format!("{figure} doesn't move that way")) 
                };

                let new_state = if board.get(to).is_some() { 
                    board.without(from).without(to).with(figure, color, to).with_another_player() 
                } else { 
                    board.without(from).with(figure, color, to).with_another_player()
                };

                Ok(new_state)
            }

            #[rustfmt::skip]
            fn get_figure_valid_moves(&self, board: &BoardState, from: Cell) -> Vec<Cell> {
                let (figure, color) = board.get(from).unwrap();

                // 1. All paths as if there are no any other figures
                let paths: Vec<Path> = match figure {
                    Figure::Knight => skip_empty(vec![
                        vec![from + (2, 1)],   vec![from + (2, -1)],   vec![from + (-2, 1)],   vec![from + (-2, -1)],
                        vec![from + (1, 2)],   vec![from + (1, -2)],   vec![from + (-1, 2)],   vec![from + (-1, -2)]
                    ]),
                    Figure::King => skip_empty(vec![
                        vec![from + (0, 1)],   vec![from + (1, 0)],    vec![from + (-1, 0)],   vec![from + (0, -1)],
                    ]),
                    Figure::Rook => on_straights(from),
                    Figure::Bishop => on_diags(from),
                    Figure::Queen => [ &on_straights(from)[..], &on_diags(from)[..] ].concat(),
                    Figure::Pawn => vec![], // TODO!
                };

                // 2. Each non-empty path - only up to the first figure on the path
                let limited_paths: Vec<Path> = paths.iter()
                    .filter(|p| p.len() > 0)
                    .map(|p| p.split_inclusive(|&c| 
                        board.get(c).is_some()).next().unwrap().to_vec()).collect();

                // 3. Don't eat ours
                let empty_or_enemy_ending_paths: Vec<Path> = limited_paths.iter().map(|p|
                    p.into_iter()
                        .filter(|&to| !matches!(board.get(*to), Some((_, c)) if c == color))
                        .map(|&c| c).collect::<Path>())
                    .collect();

                empty_or_enemy_ending_paths.into_iter().flatten().collect()
            }
        }
     }
}

fn skip_empty(v: Vec<Vec<Option<Cell>>>) -> Vec<Path> {
    v.into_iter().map(|x| skip_none(x)).filter(|p| p.len() > 0).collect()
}

fn skip_none(v: Vec<Option<Cell>>) -> Path {
    v.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
}

impl Sub<Cell> for Cell {
    type Output = Shift;

    fn sub(self, rhs: Cell) -> Self::Output {
        let l_h = self.h;
        let r_h = rhs.h;
        let l_v = self.v as u8 - board::V_LEFT as u8;
        let r_v = rhs.v as u8 - board::V_LEFT as u8;
        (l_v as i8 - r_v as i8, l_h as i8 - r_h as i8)
    }
}

impl Add<Shift> for Cell {
    type Output = Option<Cell>;

    fn add(self, rhs: Shift) -> Self::Output {
        let (dv, dh) = rhs;
        let v = (self.v as u8).wrapping_add_signed(dv) as char;
        let h = self.h.wrapping_add_signed(dh);
        match Cell::is_valid(v, h) {
            true => Some(Cell::at(v, h)),
            false => None
        }
    }
}

#[rustfmt::skip]
fn on_straights(from: Cell) -> Vec<Path> {
    vec![
        on_dir(from, ( 0,  1)),
        on_dir(from, ( 1,  0)),
        on_dir(from, ( 0, -1)),
        on_dir(from, (-1,  0))
    ]    
}

#[rustfmt::skip]
fn on_diags(from: Cell) -> Vec<Path> {
    vec![
        on_dir(from, ( 1,  1)),
        on_dir(from, ( 1, -1)),
        on_dir(from, (-1,  1)),
        on_dir(from, (-1, -1)),
    ]
}

fn on_dir(from: Cell, dir: Shift) -> Path {
    let mut cells = vec![];
    let mut cur = from;
    loop {
        match cur + dir {
            Some(next) => { cells.push(next); cur = next },
            None => return cells
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn knight_at_corner_of_empty_board() {
        let cell = Cell::at('A', 1);
        let board = board::empty().with(Figure::Knight, Color::White, cell);
        let rules_engine = RulesEngine::new();
        let moves: HashSet<Cell> = HashSet::from_iter(rules_engine.borrow().get_figure_valid_moves(&board, cell).into_iter());
        assert_eq!(
            moves, 
            HashSet::from_iter(vec![
                Cell::at('B', 3), Cell::at('C', 2)
            ])
        );
    }

}