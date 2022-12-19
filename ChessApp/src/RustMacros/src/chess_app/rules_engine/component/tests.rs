use super::*;
use Piece::*;
use Color::*;
use Condition::*;

#[test]
fn simple_white_check() {
    let board = board::empty()
        .with(King, White, Cell::at('A', 1))
        .with(King, Black, Cell::at('H', 8))
        .with(Queen, White, Cell::at('H', 1))
        .with_next_player(Black);
    let gh = GameHistory::new(board, false);
    let re = RulesEngine::new();
    let conditions = re.borrow().get_conditions(&gh);
    assert_eq!(conditions, HashSet::from([(Check, Black)]));
}

#[test]
fn simple_white_mate() {
    let board = board::empty()
        .with(King, White, Cell::at('A', 1))
        .with(King, Black, Cell::at('H', 8))
        .with(Queen, White, Cell::at('H', 1))
        .with(Rook, White, Cell::at('A', 8))
        .with_next_player(Black);
    let gh = GameHistory::new(board, false);
    let re = RulesEngine::new();
    let conditions = re.borrow().get_conditions(&gh);
    assert_eq!(conditions, HashSet::from([(Check, Black), (Mate, Black)]));
}

#[test]
fn valid_moves_of_knight_at_corner_of_empty_board() {
    let cell = Cell::at('A', 1);
    let board = board::empty().with(Knight, White, cell);
    let rules_engine = RulesEngine::new();
    let moves: HashSet<Cell> = HashSet::from_iter(rules_engine.borrow().get_figure_valid_moves(&board, cell).into_iter());
    assert_eq!(
        moves, 
        HashSet::from([
            Cell::at('B', 3), Cell::at('C', 2)
        ])
    );
}
