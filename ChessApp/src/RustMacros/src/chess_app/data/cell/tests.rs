use super::*;

#[test]
fn test_parse_cell() {
    let c = Cell::try_from("A1").unwrap();
    assert_eq!(c.v, 'A');
    assert_eq!(c.h, 1);

    let c = Cell::try_from("E2").unwrap();
    assert_eq!(c.v, 'E');
    assert_eq!(c.h, 2);
}
