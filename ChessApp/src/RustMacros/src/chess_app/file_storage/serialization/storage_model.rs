use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::chess_app::data::{Color, piece, BoardState, board, Cell, game_history, Turn};

#[derive(Serialize, Deserialize)]
pub(super) struct Game {
    pub player_color: String,
    pub game_history: GameHistory
}

#[derive(Serialize, Deserialize)]
pub(super) struct GameHistory {
    pub is_finished: bool,
    pub turns: Vec<String>,
    pub states: Vec<State>
}

#[derive(Serialize, Deserialize)]
pub(super) struct State {
    pub figures: HashMap<String, Figure>,
    pub next_player_color: String
}

#[derive(Serialize, Deserialize)]
pub(super) struct Figure {
    pub figure: String,
    pub color: String
}

impl From<(Color, &game_history::GameHistory)> for Game {
    fn from(g: (Color, &game_history::GameHistory)) -> Self {
        let (c, gh) = g;
        let player_color = c.to_string();
        let game_history = GameHistory::from(gh);
        Game { player_color, game_history }
    }
}

impl From<Game> for (Color, game_history::GameHistory) {
    fn from(g: Game) -> Self {
        let color = Color::try_from(g.player_color.as_str()).unwrap();
        let game_history = game_history::GameHistory::from(g.game_history);
        (color, game_history)
    }
}

impl From<(piece::Piece, Color)> for Figure {
    fn from(x: (piece::Piece, Color)) -> Self {
        let (figure, color) = x;
        let (figure, color) = (figure.to_string(), color.to_string());
        Figure { figure, color }
    }
}

impl From<Figure> for (piece::Piece, Color) {
    fn from(x: Figure) -> Self {
        (piece::Piece::try_from(x.figure.as_str()).unwrap(), Color::try_from(x.color.as_str()).unwrap())
    }
}

impl From<&BoardState> for State {
    fn from(bs: &BoardState) -> Self {
        let figures = HashMap::from_iter(bs.figures().iter().map(
            |(cell, (figure, color))| (cell.to_string(), (*figure, *color).into()))
        );
        let next_player_color = bs.next_player_color().to_string();

        State { figures, next_player_color }
    }
}

impl From<&State> for BoardState {
    fn from(state: &State) -> Self {
        let mut bs = board::empty();
        for (c, f) in state.figures.iter() {
            let figure = piece::Piece::try_from(f.figure.as_str()).unwrap();
            let color = Color::try_from(f.color.as_str()).unwrap();
            let cell = Cell::try_from(c.as_str()).unwrap();
            bs = bs.with(figure, color, cell);
        }
        bs.with_next_player(Color::try_from(state.next_player_color.as_str()).unwrap())
    }
}

impl From<&game_history::GameHistory> for GameHistory {
    fn from(gh: &game_history::GameHistory) -> Self {
        let is_finished = gh.is_finished();
        let turns = gh.turns().iter().map(|t| t.to_string()).collect();
        let states = gh.states().iter().map(|s| s.into()).collect();
        GameHistory { is_finished, turns, states }
    }
}

impl From<GameHistory> for game_history::GameHistory {
    fn from(gh: GameHistory) -> Self {
        let initial = BoardState::from(gh.states.first().unwrap());
        let mut history = game_history::GameHistory::new(initial, false);

        if gh.turns.len() > 0 {
            for turn_ndx in 0..=gh.turns.len() - 1 {
                let turn = Turn::try_from(gh.turns[turn_ndx].as_str()).unwrap();
                let state = BoardState::from(&gh.states[turn_ndx + 1]);
                let is_finished = gh.is_finished && turn_ndx == gh.turns.len() - 1;
                history = history.with(turn, state, is_finished);

            }
        }
        
        history  
    }
}
