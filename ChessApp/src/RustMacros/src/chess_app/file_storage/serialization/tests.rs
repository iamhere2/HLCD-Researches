use crate::chess_app::{data::{Color, GameHistory, board, Turn, Cell, Piece}, file_storage::serialization::serialize};

use super::deserialize;

#[test]
fn test_initial_serde() {
    let gh = GameHistory::classic_initial();
    let s = serialize(&gh, Color::Black).unwrap();

    let (gh, color) = deserialize(s.as_str()).unwrap();
    assert_eq!(color, Color::Black, "expected color is Black");
    assert_eq!(gh.get_state(0), &board::classic_initial());
}

#[test]
fn test_e2e4_serde() {
    let gh = GameHistory::classic_initial()
        .with(
            Turn::Move(Cell::at('E', 2), Cell::at('E', 4)), 
            board::classic_initial()
                .without(Cell::at('E', 2))
                .with(Piece::Pawn, Color::White, Cell::at('E', 4)),
            false);

    let s = serialize(&gh, Color::White).unwrap();

    let (gh, color) = deserialize(s.as_str()).unwrap();
    assert_eq!(color, Color::White, "expected color is White");
    
    assert_eq!(gh.states().len(), 2);
    assert_eq!(gh.turns().len(), 1);
}