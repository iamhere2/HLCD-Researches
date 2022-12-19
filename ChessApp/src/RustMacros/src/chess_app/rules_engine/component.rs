#[cfg(test)]
mod tests;

use std::{ops::{Add, Sub}, collections::HashSet};
use strum::IntoEnumIterator;
use crate::chess_app::data::{Color, Cell, Turn, RuleViolation, BoardState, Piece, GameHistory, board};
use super::interface::*;

type Shift = (i8, i8);
type Path = Vec<Cell>;

hlcd::define! {

     component RulesEngine {

        provides { RulesEngine }

        impl RulesEngine {
            fn apply(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<BoardState, RuleViolation> {
                match turn {
                    Turn::Move(from, to) => self.apply_move(bs, player, from, to),
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
                let board = gh.current_state();
                self.get_valid_turns_for(gh, board.next_player_color())
            }

            fn get_conditions(&self, gh: &GameHistory) -> HashSet<(Condition, Color)> {
                let mut conditions = HashSet::new();

                for color in Color::iter() {
                    if self.is_check(gh, color) {
                        conditions.insert((Condition::Check, color));

                        // For mate condition, any valid turn should keep the Check condition
                        let board = gh.current_state().with_next_player(color);
                        let turns = self.get_valid_turns_for(gh, color);
                        let mut is_mate = true;

                        for &t in turns.iter() {
                            let board_after = self.apply(&board, color, t).unwrap();
                            let gh_after = gh.with(t, board_after, false);
                            if !self.is_check(&gh_after, color) {
                                is_mate = false;
                                break;
                            }
                        }

                        if is_mate { 
                            conditions.insert((Condition::Mate, color));
                        }
                    }
                }
                
                conditions
            }
        }

        impl {

            fn get_valid_turns_for(&self, gh: &GameHistory, player: Color) -> Vec<Turn> {
                let board = gh.current_state();
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

            fn is_check(&self, gh: &GameHistory, color: Color) -> bool {
                let turns = self.get_valid_turns_for(gh, !color);
                let board = gh.current_state();
                let king = Some((Piece::King, color));
                turns.iter().any(|t| matches!(t, Turn::Move(_, to) if board.get(*to) == king))
            }

            fn apply_move(&self, board: &BoardState, player: Color, from: Cell, to: Cell) -> Result<BoardState, String> {
                if board.next_player_color() != player {
                    return Err("It's another player's turn now".to_string()) 
                };

                let (figure, color) = board.get(from)
                    .ok_or("No figure at source cell")?;

                if color != player { 
                    return Err("Can't move figures of other color".to_string()) 
                };

                let valid_moves = self.get_figure_valid_moves(board, from);
                if valid_moves.iter().position(|c| *c == to).is_none() {
                    return Err(format!("{} at {} isn't allowed to move/eat {}", figure, from, to)); 
                }

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

                let any_at = |s: Shift| -> bool {
                    let at = from + s;
                    at.is_some() && board.get(at.unwrap()).is_some()
                };

                // 1. All paths as if there were no any other figures
                let paths: Vec<Path> = match figure {
                    Piece::Knight => skip_empty([
                        [from + (2, 1)],  [from + (2, -1)],  [from + (-2, 1)],  [from + (-2, -1)],
                        [from + (1, 2)],  [from + (1, -2)],  [from + (-1, 2)],  [from + (-1, -2)]
                    ].map(|a| a.to_vec()).into_iter().collect()),
                    Piece::King => skip_empty([
                        [from + (0, 1)],  [from + (1,  0)],  [from + (-1, 0)],  [from + ( 0, -1)],
                    ].map(|a| a.to_vec()).into_iter().collect()),
                    Piece::Rook => on_straights(from),
                    Piece::Bishop => on_diags(from),
                    Piece::Queen => [ &on_straights(from)[..], &on_diags(from)[..] ].concat(),
                    Piece::Pawn => skip_empty(vec![
                        match (color, from.h) {
                            (Color::White, 2) if !any_at((0,  2)) => vec![from + (0,  2)],
                            (Color::Black, 7) if !any_at((0, -2)) => vec![from + (0, -2)],
                            _ => vec![]
                        },
                        match color {
                            Color::White if !any_at((0,  1)) => vec![from + (0,  1)],
                            Color::Black if !any_at((0, -1)) => vec![from + (0, -1)],
                            _ => vec![]
                        },
                        match color {
                            Color::White if any_at(( 1,  1)) => vec![from + ( 1,  1)],
                            Color::Black if any_at(( 1, -1)) => vec![from + ( 1, -1)],
                            _ => vec![]
                        },
                        match color {
                            Color::White if any_at((-1,  1)) => vec![from + (-1,  1)],
                            Color::Black if any_at((-1, -1)) => vec![from + (-1, -1)],
                            _ => vec![]
                        }
                    ]), 
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

    while let Some(next) = cur + dir {
        cells.push(next); 
        cur = next
    }

    cells
}

