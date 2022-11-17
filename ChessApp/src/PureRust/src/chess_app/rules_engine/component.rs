use std::{cell::RefCell, rc::Rc};

use crate::chess_app::data::{Color, Cell, Turn, RuleViolation, BoardState};

use super::interface::*;

pub struct RulesEngine { }

impl RulesEngine {
    pub fn new() -> RulesEngine {
        RulesEngine {  }
    }
}

fn check_move(bs: &BoardState, player: Color, from: Cell, to: Cell) -> Result<(), String> {
    let (figure, color) = bs.get(from).ok_or("No figure at source cell")?;
    if color != player { return Err("Can't move figures of other color".to_string()) };
    Ok(())
}

impl RulesEngineInterface for RulesEngine {
    fn check(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<(), RuleViolation> {
        match turn {
            Turn::Move(from, to) => check_move(bs, player, from, to),
            Turn::PawnTransform(_, _) => unimplemented!(),
            Turn::Draw => todo!(),
            Turn::Reject => todo!(),
            Turn::Castle(_) => todo!(),
        }.map_err(|e| RuleViolation::new(bs.clone(), player, turn, e))
    }

    fn apply(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<BoardState, RuleViolation> {
        dbg!("Applying turn...");
        Ok(bs.clone())
    }
}


impl RulesEngineProvider for RulesEngine {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn RulesEngineInterface>> {
        it
    }
}