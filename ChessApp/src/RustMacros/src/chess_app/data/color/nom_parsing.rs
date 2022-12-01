use super::*;
use nom_extensions::parseable::Parseable;
use nom::{branch::alt, combinator::map, bytes::complete::tag_no_case, IResult};

impl Parseable for Color {
    
    fn nom_parse(input: &str) -> IResult<&str, Self> {
        let white = map(tag_no_case("white"), |_| Color::White);
        let black = map(tag_no_case("black"), |_| Color::Black);
        alt((black, white))
        (input)
    }

    const MESSAGE: &'static str = "expected color: black|white";
}
