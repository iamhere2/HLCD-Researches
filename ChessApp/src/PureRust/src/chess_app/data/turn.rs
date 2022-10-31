use std::fmt::{Display, format};

use super::{Cell, Figure};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Turn {
    Move(Cell, Cell),
    PawnTransform(Cell, Figure),
    Draw,
    Reject,
    Castle(Cell)
}

#[rustfmt::skip]
impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            Turn::Move(from, to)                => format!("{from} - {to}"),
            Turn::PawnTransform(from, figure)   => format!("Pawn {from} -> {figure}"),
            Turn::Draw                          => format!("Draw"),
            Turn::Reject                        => format!("Reject"),
            Turn::Castle(to)                    => format!("Castle {to}")
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct ParseError(String);

use nom::{
    IResult, 
    branch::alt, 
    bytes::complete::{tag, tag_no_case}, 
    combinator::{map, recognize}, 
    sequence::{separated_pair, tuple}, 
    character::complete::{char, alpha1, multispace1, space1, one_of, space0}, 
    multi::many1};

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    let cell = Cell::try_from(&input[..2]).map_err(
        |e| nom::Err::Error(nom::error::Error{ input, code: nom::error::ErrorKind::Fail }))?;
    Ok((&input[2..], cell))
}

pub fn parse_turn(input: &str) -> IResult<&str, Turn> {
    let mut move_turn = map(tuple((parse_cell, space0, char('-'), space0, parse_cell)),
        |(from, _, _, _, to)| Turn::Move(from, to)); 

    move_turn
    (input)
}

impl TryFrom<&str> for Turn {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_turn(value)
            .map(|(_, t)| t)
            .map_err(|e| ParseError(format!("{e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell() {
        let r = parse_cell("E2");
        assert_eq!(r, Ok(("", Cell::at('E', 2))));

        let r = parse_cell("E2123");
        assert_eq!(r, Ok(("123", Cell::at('E', 2))));
    }

    #[test]
    fn test_parse_move() {
        let m = Turn::try_from("E2 - E4").unwrap();
        assert_eq!(m, Turn::Move(Cell::at('E', 2), Cell::at('E', 4)));
    }
}