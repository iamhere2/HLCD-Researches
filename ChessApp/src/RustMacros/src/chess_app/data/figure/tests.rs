use super::*;
use nom_extensions::parseable::Parseable;

#[test]
fn test_parse() {
    assert_eq!(Figure::parse("Rook"), Ok(Figure::Rook));
}

#[test]
fn test_nom_parse() {
    assert_eq!(Figure::nom_parse("pawns"), Ok(("s", Figure::Pawn)));
}
