use std::{cell::RefCell, rc::Rc};
use crate::chess_app::data::{Color, Cell, Turn, RuleViolation, BoardState};
use super::interface::*;


hlcd::define! {

     component RulesEngine {

         requires {
//             // aaa,
//             // B: Ibbb,
         }

         provides {
//             // Aaa: ChildrenA,
//             // Bbb: ChildrenB.B,
//             // Ccc, // :self
//             // Ddd,
             RulesEngine
         }

         state {
//             // x: A, 
//             // y: B
         }

//         children {
//             // ChildrenA,
//             // ChildrenB
//         }

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
        }

         impl {
            fn apply_move(board: &BoardState, player: Color, from: Cell, to: Cell) -> Result<BoardState, String> {
                let (figure, color) = board.get(from).ok_or("No figure at source cell")?;
                if color != player { return Err("Can't move figures of other color".to_string()) };
                let new_state = board.without(from).with(figure, color, to);
                Ok(new_state)
            }
        }

     }

}



// Component
// Children: -
// Consumes: -
// State: -
// Provides: RulesEngine (own)
// pub struct RulesEngine { }

impl RulesEngine {
    pub fn new() -> RulesEngine {
        RulesEngine {  }
    }
}

// fn apply_move(board: &BoardState, player: Color, from: Cell, to: Cell) -> Result<BoardState, String> {
//     let (figure, color) = board.get(from).ok_or("No figure at source cell")?;
//     if color != player { return Err("Can't move figures of other color".to_string()) };
//     let new_state = board.without(from).with(figure, color, to);
//     Ok(new_state)
// }

// impl RulesEngineInterface for RulesEngine {
// }

