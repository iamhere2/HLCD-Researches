use super::*;
use nom_extensions::parseable::Parseable;

#[test]
fn test_parse() {
    assert_eq!(Piece::parse("Rook"), Ok(Piece::Rook));
}

#[test]
fn test_nom_parse() {
    assert_eq!(Piece::nom_parse("pawns"), Ok(("s", Piece::Pawn)));
}
