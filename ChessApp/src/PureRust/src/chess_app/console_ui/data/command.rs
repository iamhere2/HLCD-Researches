use strum::Display;

use crate::chess_app::data::{Color, Turn};

#[derive(PartialEq, Eq, Clone, Debug, Display)]
pub enum Command {
    MakeTurn(Turn),
    Exit,
    Help,
    NewGame(Color),
    ListGames,
    LoadGame(String),
    DeleteGame(String),
    SaveGame(String)
} 