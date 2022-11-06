use crate::chess_app::data::{Color, Turn};

#[derive(PartialEq, Eq, Clone, Debug)]
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