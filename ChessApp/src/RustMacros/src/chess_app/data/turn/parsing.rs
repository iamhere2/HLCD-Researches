use super::*;
use nom::{IResult, combinator::map, sequence::tuple, 
    character::complete::{char, space0}
};

impl Parseable for Turn {
    const MESSAGE: &'static str = "invalid turn; expected: {cell} - {cell} | ...";
    
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        let mut move_turn = map(tuple((Cell::nom_parse, space0, char('-'), space0, Cell::nom_parse)),
            |(from, _, _, _, to)| Turn::Move(from, to)); 
        
        move_turn
        (input)
    }
}
