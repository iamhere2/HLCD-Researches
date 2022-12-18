use std::collections::HashMap;

use rand::Rng;

use crate::chess_app::data::Piece;
use super::{player_interface::*, data::{Turn, GameHistory}, rules_engine::interface::*};

hlcd::define! {
    component StupidAiPlayer {
        requires { RulesEngine }
        provides { PassiveSyncPlayer }
        impl PassiveSyncPlayer {
            fn turn_request(&self, gh: &GameHistory) -> Turn {
                let state = gh.current_state();
                let player = state.next_player_color();
                let valid_turns = self.rules_engine().get_valid_turns(gh);
                let turns_with_ranks: HashMap<Turn, i32> = HashMap::from_iter(
                    valid_turns.into_iter().map(|t|{
                        let board_after = { self.rules_engine().apply(state, player, t).unwrap() };
                        let gh_after = gh.with(t, board_after, false);    
                        let rank = self.get_2nd_turn_position_rank(&gh_after);
                        dbg!(format!("{t}: {rank}"));
                        (t, rank)
                    }));
                
                // select random turn from the highest ranks
                let max_rank = *turns_with_ranks.values().max().unwrap();
                let max_rank_turns = turns_with_ranks.iter()
                    .filter(|(_, &r)| r == max_rank)
                    .map(|(&t, _)| t)
                    .collect::<Vec<_>>();
                let mut rng = rand::thread_rng();
                let rnd_ndx = rng.gen_range(0..max_rank_turns.len());
                max_rank_turns[rnd_ndx]
            }
        }

        impl {
            fn get_2nd_turn_position_rank(&self, gh: &GameHistory) -> i32 {
                let board = gh.current_state();
                let player = board.next_player_color();
                let valid_opponent_turns = { self.rules_engine().get_valid_turns(gh) };
                valid_opponent_turns.into_iter().map(|t|{
                    let board_after = { self.rules_engine().apply(board, player, t).unwrap() };
                    let gh_after = gh.with(t, board_after, false);
                    self.get_position_rank(&gh_after)
                }).max().unwrap()
            }

            #[rustfmt::skip]
            fn get_position_rank(&self, gh: &GameHistory) -> i32 {
                use Piece::*;
                let board = gh.current_state();
                let other_player = board.next_player_color();
                let figures_rank = board.figures().values().into_iter().fold(0, |acc, (f, c)|
                    acc + match f {
                        King   => 0,
                        Queen  => 10,
                        Bishop => 8,
                        Rook   => 7,
                        Knight => 5,
                        Pawn   => 1
                    } * (if *c == other_player { 1 } else { -1 })
                );
                
                let conditions = self.rules_engine().get_conditions(gh);
                let conditions_rank = conditions.into_iter().fold(0, |acc, (cond, c)|
                    acc + match cond {
                        Condition::Check => 100,
                        Condition::Mate => 1000,
                    } * (if c == other_player { -1 } else { 1 })
                );

                figures_rank + conditions_rank
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chess_app::{rules_engine::{component::RulesEngine, interface::RulesEngineProvider}, data::{board, GameHistory, Cell, Piece, Color}, ai_player::StupidAiPlayer};

    #[test]
    fn initial_position_rank_is_zero() {
        let board = board::classic_initial();
        let gh = GameHistory::new(board, false);
        let rules_engine = RulesEngineProvider::get(RulesEngine::new());
        let ai = StupidAiPlayer::new(&rules_engine);
        let rank = ai.borrow().get_position_rank(&gh);
        assert_eq!(rank, 0);
    }

    #[test]
    fn more_figures_means_better_rank() {
        let board = board::empty()
            .with(Piece::Bishop, Color::White, Cell::at('A', 1))
            .with(Piece::Pawn, Color::Black, Cell::at('H', 8))
            .with_next_player(Color::White);
        let gh = GameHistory::new(board, false);
        let rules_engine = RulesEngineProvider::get(RulesEngine::new());
        let ai = StupidAiPlayer::new(&rules_engine);
        let rank = ai.borrow().get_position_rank(&gh);
        assert_eq!(rank, 7);
    }

    #[test]
    fn check_means_better_rank() {
        let board = board::empty()
            .with(Piece::Rook, Color::White, Cell::at('A', 1))
            .with(Piece::Rook, Color::Black, Cell::at('H', 8))
            .with(Piece::King, Color::Black, Cell::at('A', 8))
            .with_next_player(Color::White);
        let gh = GameHistory::new(board, false);
        let rules_engine = RulesEngineProvider::get(RulesEngine::new());
        let ai = StupidAiPlayer::new(&rules_engine);
        let rank = ai.borrow().get_position_rank(&gh);
        assert_eq!(rank, 100);
    }
}