use super::*;

#[test]
fn test_parse_move() {
    let m = Turn::try_from("E2 - E4").unwrap();
    assert_eq!(m, Turn::Move(Cell::at('E', 2), Cell::at('E', 4)));
}
