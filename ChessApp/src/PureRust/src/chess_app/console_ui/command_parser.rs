#[cfg(test)]
mod tests;

use std::{rc::Rc, cell::RefCell};

use nom::{
    IResult, 
    error::{ErrorKind, ParseError}, 
    branch::alt, 
    bytes::complete::{tag, tag_no_case},    
    combinator::{map, recognize}, 
    sequence::{separated_pair, tuple}, 
    character::complete::{char, alpha1, multispace1, space1, one_of, space0}, 
    multi::many1};

use crate::{chess_app::data::{Figure, Color, Cell, Turn}, nom_extensions::parseable::Parseable};

use super::{command_parser_interfaces::{
        CommandParserProvider, CommandParserInterface, Error
    }, 
    data::command::Command
};

pub(super) struct CommandParser {
}

impl CommandParser {
    pub fn new() -> CommandParser {
        CommandParser {  }
    }

    fn ident(input: &str) -> IResult<&str, &str> {
        recognize(many1(alpha1))(input)
    }

    fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, Command> {
        let ident = Self::ident;
        let color = Color::nom_parse;
        let cell = Cell::nom_parse;

        let exit = map(tag_no_case("exit"), |_| Command::Exit);
        let list = map(tag_no_case("list"), |_| Command::ListGames);

        let load = map(separated_pair(tag_no_case("load"), space1, ident), 
            |(_, name)| Command::LoadGame(name.to_string()));
        
        let save = map(separated_pair(tag_no_case("save"), space1, ident), 
            |(_, name)| Command::SaveGame(name.to_string()));
        
        let del = map(separated_pair(tag_no_case("del"), space1, ident), 
            |(_, name)| Command::DeleteGame(name.to_string()));
        
        let new = map(separated_pair(tag_no_case("new"), space1, color), 
            |(_, color)| Command::NewGame(color));

        let move_turn = map(tuple((cell, space0, char('-'), space0, cell)),
            |(from, _, _, _, to)| Command::MakeTurn(Turn::Move(from, to))); 

        alt((exit, list, load, save, del, new, move_turn))
        (input)
    }
}

impl CommandParserProvider for CommandParser {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn CommandParserInterface>> { it }
}

impl CommandParserInterface for CommandParser {

    fn parse(&self, s: &str) -> Result<Command, Error> {
        match self.parse(s) {
            Ok((_, cmd)) => Ok(cmd),
            Err(e) => Err(Error(format!("Invalid command. {:?}", e)))
        }
    }

    fn get_help(&self) -> String {
        r#"
            Game commands:
                save <game name>
                load <game name>
                del  <game name>
                list            - lists saved games
                new <color>     - starts new game, playing with black or white
                exit            - exit
                
            Turns:
                <from cell> - <to cell>   - move or eat
                    e.g.: E2 - E4
                
                <from cell> - <to figure> - pawn transformation
                    e.g.: D7 - Queen
                
                castle - <to cell>        - castle
                    e.g.: castle - A7
                
                draw                      - offer/accept draw
                reject                    - reject draw
                    
        "#.to_string()
    }
}

