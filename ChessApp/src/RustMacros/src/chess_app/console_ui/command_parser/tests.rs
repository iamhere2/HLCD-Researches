use std::cell::RefCell;

use crate::chess_app::{data::{Turn, Color, Cell}, console_ui::data::command::Command};

use super::{interface::{CommandParserInterface, Error}, component::CommandParser};

fn parse(s: &str) -> Result<Command, Error> {
    let parser = CommandParser::new();
    let parser = RefCell::borrow(&parser);
    parser.parse(s)
}

#[test]
fn test_parse() {
    use Command::*;
    assert_eq!(parse("exit"), Ok(Exit));
    assert_eq!(parse("Exit"), Ok(Exit));
    assert_eq!(parse("List"), Ok(ListGames));
    assert!(matches!(parse("NotACommand"), Err(Error(_))));
    assert_eq!(parse("load AAA"), Ok(LoadGame("AAA".to_string())));
    assert_eq!(parse("save AAA"), Ok(SaveGame("AAA".to_string())));
    assert_eq!(parse("del AAA"), Ok(DeleteGame("AAA".to_string())));
    assert_eq!(parse("new Black"), Ok(NewGame(Color::Black)));
    assert_eq!(parse("E2 - E4"), Ok(MakeTurn(Turn::Move(Cell::at('E', 2), Cell::at('E', 4)))));
    assert_eq!(parse("g3-h7"), Ok(MakeTurn(Turn::Move(Cell::at('G', 3), Cell::at('H', 7)))));
}
