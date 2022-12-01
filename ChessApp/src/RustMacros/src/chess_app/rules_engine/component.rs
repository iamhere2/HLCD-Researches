use crate::chess_app::data::{Color, Cell, Turn, RuleViolation, BoardState, Figure};
use super::interface::*;

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
        }

        impl {
            fn apply_move(board: &BoardState, player: Color, from: Cell, to: Cell) -> Result<BoardState, String> {
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

                let dh = to.v_num() as i8 - from.v_num() as i8;
                let dv = to.h as i8 - from.h as i8;

                let can_move_that_way = 
                    match figure {
                        Figure::King => dh == 0 && dv.abs() == 1 || dh.abs() == 1 && dv == 0,
                        Figure::Rook => dh == 0 || dv == 0,
                        Figure::Queen => dh == 0 || dv == 0 || dh.abs() == dv.abs(),
                        Figure::Bishop => dh.abs() == dv.abs(),
                        Figure::Pawn => 
                               (color == Color::White && dh == 0 && dv ==  1 && to_figure.is_none())
                            || (color == Color::Black && dh == 0 && dv == -1 && to_figure.is_none())
                            || (color == Color::White && from.h == 2 && dh == 0 && dv ==  2 && to_figure.is_none())
                            || (color == Color::Black && from.h == 7 && dh == 0 && dv == -2 && to_figure.is_none())
                            || (color == Color::White && dh.abs() == 1 && dv ==  1 && to_figure.is_some())
                            || (color == Color::Black && dh.abs() == 1 && dv == -1 && to_figure.is_some()),
                        Figure::Knight => 
                               dh.abs() == 2 && dv.abs() == 1
                            || dh.abs() == 1 && dv.abs() == 2
                    };

                if !can_move_that_way { 
                    return Err(format!("{figure} doesn't move that way")) 
                };

                let new_state = if board.get(to).is_some() { 
                    board.without(from).without(to).with(figure, color, to) 
                } else { 
                    board.without(from).with(figure, color, to)
                };

                Ok(new_state)
            }
        }
     }
}