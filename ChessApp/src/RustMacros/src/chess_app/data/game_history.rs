use super::{BoardState, Turn, board};

pub struct GameHistory { 
    is_finished: bool,
    states: Vec<BoardState>,
    turns: Vec<Turn>
}

impl GameHistory {
    pub fn new(state: BoardState, is_finished: bool) -> GameHistory {
        GameHistory {  
            is_finished,
            states: vec![ state ],
            turns: Vec::new()
        }
    }

    pub fn classic_initial() -> GameHistory {
        Self::new(board::classic_initial(), false)
    }

    pub fn with(&self, turn: Turn, state: BoardState, is_finished: bool) -> GameHistory {
        if self.is_finished { panic!("GameHistory is already finished"); }

        let mut states = self.states.clone();
        let mut turns = self.turns.clone();

        states.push(state);
        turns.push(turn);

        GameHistory { is_finished, states, turns }
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn turn_count(&self) -> u32 {
        self.turns.len() as u32
    }

    pub fn get_turn(&self, i: u32) -> Turn {
        self.turns[i as usize]
    }

    pub fn get_state(&self, before_turn_index: u32) -> &BoardState {
        &self.states[before_turn_index as usize]
    }

    pub fn turns(&self) -> &Vec<Turn> {
        &self.turns
    }

    pub fn states(&self) -> &Vec<BoardState> {
        &self.states
    }
}
