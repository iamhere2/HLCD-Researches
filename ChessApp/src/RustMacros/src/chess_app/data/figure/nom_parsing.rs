use super::*;
use crate::nom_extensions::parseable::Parseable;
use nom::{IResult, combinator::map, branch::alt, bytes::complete::tag_no_case};

impl Parseable for Figure {
    const MESSAGE: &'static str = "invalid figure; expected one of: Knight, King, Rook, Bishop, Queen, Pawn";

    #[rustfmt::skip]
    fn nom_parse(input: &str) -> IResult<&str, Self> {

        alt((
            map(tag_no_case("Knight"), |_| Figure::Knight), 
            map(tag_no_case("King"),   |_| Figure::King), 
            map(tag_no_case("Rook"),   |_| Figure::Rook), 
            map(tag_no_case("Bishop"), |_| Figure::Bishop), 
            map(tag_no_case("Queen"),  |_| Figure::Queen), 
            map(tag_no_case("Pawn"),   |_| Figure::Pawn)
        ))
        (input)
    }
}
