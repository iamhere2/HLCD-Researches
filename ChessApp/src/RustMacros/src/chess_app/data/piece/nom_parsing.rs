use super::*;
use nom_extensions::parseable::Parseable;
use nom::{IResult, combinator::map, branch::alt, bytes::complete::tag_no_case};

impl Parseable for Piece {
    const MESSAGE: &'static str = "invalid figure; expected one of: Knight, King, Rook, Bishop, Queen, Pawn";

    #[rustfmt::skip]
    fn nom_parse(input: &str) -> IResult<&str, Self> {

        alt((
            map(tag_no_case("Knight"), |_| Piece::Knight),
            map(tag_no_case("King"),   |_| Piece::King), 
            map(tag_no_case("Rook"),   |_| Piece::Rook), 
            map(tag_no_case("Bishop"), |_| Piece::Bishop), 
            map(tag_no_case("Queen"),  |_| Piece::Queen), 
            map(tag_no_case("Pawn"),   |_| Piece::Pawn)
        ))
        (input)
    }
}
