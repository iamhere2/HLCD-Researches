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

impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        let cs: Vec<_> = s.chars().collect();
        if cs.len() != 2 { panic!("Invalid cell {s}") };
        let v = cs[0];
        let h = cs[1];
        Cell::at(v, h.to_digit(10).unwrap() as u8)
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
        let c = Cell::from("A1");
        assert_eq!(c.v, 'A');
        assert_eq!(c.h, 1);

        let c = Cell::from("E2");
        assert_eq!(c.v, 'E');
        assert_eq!(c.h, 2);
    }
}