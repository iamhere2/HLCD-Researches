#[cfg(test)]
mod tests;

pub mod interface {
    use std::fmt::Display;
    use super::super::data::command::Command;
    
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct Error(pub String); 
    
    impl std::error::Error for Error{
    }
    
    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    hlcd::define! {
        interface CommandParser {
            fn parse(&self, s: &str) -> Result<Command, Error>;
            fn get_help(&self) -> String;
        }
    }
}

pub mod component {
    use nom::{
        IResult, 
        branch::alt, 
        bytes::complete::tag_no_case,    
        combinator::{map, recognize}, 
        sequence::separated_pair, 
        character::complete::{alpha1, space1}, 
        multi::many1};
    
    use crate::{chess_app::data::{Color, Turn}};
    use nom_extensions::parseable::Parseable;
    use super::{interface::*, super::data::command::Command};
    
    hlcd::define! {
        component CommandParser {
            provides { CommandParser }
    
            impl {
                fn ident(input: &str) -> IResult<&str, &str> {
                    recognize(many1(alpha1))(input)
                }
            
                fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, Command> {
                    use Command::*;
                    let ident = Self::ident;
                    let color = Color::nom_parse;
            
                    let exit = map(tag_no_case("exit"), |_| Exit);
                    let help = map(tag_no_case("help"), |_| Help);
                    let list = map(tag_no_case("list"), |_| ListGames);
            
                    let load = map(separated_pair(tag_no_case("load"), space1, ident), 
                        |(_, name)| LoadGame(name.to_string()));
                    
                    let save = map(separated_pair(tag_no_case("save"), space1, ident), 
                        |(_, name)| SaveGame(name.to_string()));
                    
                    let del = map(separated_pair(tag_no_case("del"), space1, ident), 
                        |(_, name)| DeleteGame(name.to_string()));
                    
                    let new = map(separated_pair(tag_no_case("new"), space1, color), 
                        |(_, color)| NewGame(color));
            
                    let turn = map(Turn::nom_parse,
                        |turn| MakeTurn(turn)); 
            
                    alt((exit, help, list, load, save, del, new, turn))
                    (input)
                }    
            }
    
            impl CommandParser {
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
        }
    }
}
