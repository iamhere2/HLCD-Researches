use super::*;
use super::nom_parsing;
use crate::nom_extensions::parseable::Parseable;

#[test]
fn test_invert() {
    assert_eq!(!Color::Black, Color::White);
    assert_eq!(!Color::White, Color::Black);
}

#[test]
fn test_parse() {
    assert_eq!(Color::parse("WHITE"), Ok(Color::White));
}

#[test]
fn test_nom_parse() {
    assert_eq!(Color::nom_parse("blackish"), Ok(("ish", Color::Black)));
}
