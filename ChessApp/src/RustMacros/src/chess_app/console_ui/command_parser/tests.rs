use crate::chess_app::{data::{Turn, Color, Cell}, console_ui::data::command::Command};

use super::{*, interface::{CommandParserInterface, Error}, component::CommandParser};

fn parse(s: &str) -> Result<Command, Error> {
    CommandParserInterface::parse(&CommandParser::new(), s)
}

#[test]
fn test_parse() {
    assert_eq!(parse("exit"), Ok(Command::Exit));
    assert_eq!(parse("Exit"), Ok(Command::Exit));
    assert_eq!(parse("List"), Ok(Command::ListGames));
    assert!(matches!(parse("NotACommand"), Err(Error(_))));
    assert_eq!(parse("load AAA"), Ok(Command::LoadGame("AAA".to_string())));
    assert_eq!(parse("save AAA"), Ok(Command::SaveGame("AAA".to_string())));
    assert_eq!(parse("del AAA"), Ok(Command::DeleteGame("AAA".to_string())));
    assert_eq!(parse("new Black"), Ok(Command::NewGame(Color::Black)));
    assert_eq!(parse("E2 - E4"), Ok(Command::MakeTurn(Turn::Move(Cell::at('E', 2), Cell::at('E', 4)))));
    assert_eq!(parse("g3-h7"), Ok(Command::MakeTurn(Turn::Move(Cell::at('G', 3), Cell::at('H', 7)))));
}
