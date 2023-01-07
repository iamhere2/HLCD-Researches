use nom_extensions::parseable::Parseable;
use super::*;
use nom::{IResult, combinator::map, sequence::tuple, character::complete::one_of};

impl Parseable for Cell {
    const MESSAGE: &'static str = "invalid cell; expected '{V}{H}'";
    
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        map(tuple((one_of("abcdefghABCDEFGH"), one_of("12345678"))),
            |(v, h)| Cell::at(v.to_ascii_uppercase(), h.to_digit(10).unwrap() as u8))
        (input)
    }
}
