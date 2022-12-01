#[cfg(test)]
mod tests;
mod storage_model;

use crate::chess_app::data::{GameHistory, Color};

#[derive(Clone, Debug)]
pub(super) struct Error(pub String);

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error(format!("Serialization error. {:?}", e))
    }
} 

pub(super) fn serialize(game_history: GameHistory, player_color: Color) -> Result<String, Error> {
    let player_color = player_color.to_string();
    let game_history = storage_model::GameHistory::from(game_history);

    let g = storage_model::Game { player_color, game_history };
    let s = serde_json::to_string(&g)?;
    Ok(s)
}

pub(super) fn deserialize(s: &str) -> Result<(GameHistory, Color), Error> {
    let game: storage_model::Game = serde_json::from_str(s)?;
    let color = Color::try_from(game.player_color.as_str())
        .map_err(|e| Error(format!("{e:?}")))?;

    let gh = GameHistory::from(game.game_history);
    Ok((gh, color))
}
