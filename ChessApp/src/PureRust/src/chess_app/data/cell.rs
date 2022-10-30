use std::fmt::Display;

use super::board;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[readonly::make]
pub struct Cell {
    pub v: char,
    pub h: u8
}

impl Cell {

    pub fn is_valid(v: char, h: u8) -> bool {
        v >= board::V_LEFT && v <= board::V_RIGHT && h >= board::H_BOTTOM && h <= board::H_TOP
    }

    pub fn at(v: char, h: u8) -> Cell {
        if !Cell::is_valid(v, h) { panic!("Invalid cell {v}{h}") };
        Cell {v, h}
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.v, self.h)
    }
}

#[derive(Debug)]
pub struct ParseError(String);

impl TryFrom<&str> for Cell {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let cs: Vec<_> = s.chars().collect();
        if cs.len() != 2 { return Err(ParseError(format!("Invalid cell {s}"))); };
        let v = cs[0];
        let h = cs[1];
        Ok(Cell::at(v, h.to_digit(10).unwrap() as u8))
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.v, self.h)
    }
}

#[cfg(test)]
mod tests {
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
}