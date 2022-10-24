#[cfg(test)]
mod tests;

use crate::chess_app::data::{GameHistory, Color, game_history};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub(super) struct Error(pub String);

#[derive(Serialize, Deserialize)]
struct StorageModel {
    player_color: Color,
    game_history: GameHistory
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error(format!("Serialization error. {}", e))
    }
} 

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        todo!()
    }
}

impl Serialize for GameHistory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer 
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for GameHistory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        todo!()
    }
}

pub(super) fn serialize(game_history: GameHistory, player_color: Color) -> Result<String, Error> {
    let m = StorageModel { game_history, player_color };
    let s = serde_json::to_string(&m)?;
    Ok(s)
}

pub(super) fn deserialize(s: &str) -> Result<(GameHistory, Color), Error> {
    let m: StorageModel = serde_json::from_str(s)?;
    Ok((m.game_history, m.player_color))
}
