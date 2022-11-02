use super::*;
use crate::nom_extensions::parseable::{Parseable, ParseError};
use nom::{
    IResult, 
    branch::alt, 
    bytes::complete::{tag, tag_no_case}, 
    combinator::{map, recognize}, 
    sequence::{separated_pair, tuple}, 
    character::complete::{char, alpha1, multispace1, space1, one_of, space0}, 
    multi::many1};

impl Parseable for Turn {
    const MESSAGE: &'static str = "invalid turn; expected ...";
    
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        let mut move_turn = map(tuple((Cell::nom_parse, space0, char('-'), space0, Cell::nom_parse)),
            |(from, _, _, _, to)| Turn::Move(from, to)); 
        
        move_turn
        (input)
    }
}
