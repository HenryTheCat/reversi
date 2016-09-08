//! Test module.

#![feature(test)]
extern crate test;
extern crate reversi;

use reversi::board;
use reversi::game;
use reversi::{Side, ReversiError};
use reversi::board::{Coord};
use reversi::turn::{Turn};
use reversi::game::{Game, PlayerAction};

/// A stupid class implementing `IsPlayer` to be used for testing purposes.
struct FoolPlayer;

impl game::IsPlayer<()> for FoolPlayer {
    fn make_move(&self, turn: &Turn) -> reversi::Result<PlayerAction<()>> {
        for (row, &row_array) in turn.get_board().get_all_cells().into_iter().enumerate() {
            for (col, &_) in row_array.into_iter().enumerate() {
                let coord = Coord::new(row, col);
                if turn.check_move(coord).is_ok() {
                    return Ok(PlayerAction::Move(coord));
                }
            }
        }
        Err(ReversiError::EndedGame)
    }
}

/// A simple class implementing `IsPlayer` to be used for testing purposes.
struct SimplePlayer;

impl game::IsPlayer<()> for SimplePlayer {
    fn make_move(&self, turn: &Turn) -> reversi::Result<PlayerAction<()>> {
        let mut best_move = Coord::new(0, 0);
        let mut best_score = match turn.get_state() {
            Some(Side::Dark)  => i16::max_value(),
            Some(Side::Light) => i16::min_value(),
            None => return Err(ReversiError::EndedGame),
        };

        for row in 0..board::BOARD_SIZE {
            for col in 0..board::BOARD_SIZE {
                let coord = Coord::new(row, col);
                if let Ok(new_score) = turn.make_move(coord).map(|turn_after_move| self.eval(&turn_after_move, 3)) {
                    match turn.get_state() {
                        Some(Side::Dark)  => {
                            if new_score < best_score {
                                best_move = coord;
                                best_score = new_score;
                            }
                        },
                        Some(Side::Light) => {
                            if new_score > best_score {
                                best_move = coord;
                                best_score = new_score;
                            }
                        },
                        None => return Err(ReversiError::EndedGame),
                    };
                }
            }
        }
        Ok(PlayerAction::Move(best_move))
    }
}

impl SimplePlayer {
    fn eval(&self, turn: &Turn, depth: u8) -> i16 {
        match turn.get_state() {
            None => turn.get_score_diff() * board::NUM_CELLS as i16,
            Some(side) => {
                if depth == 0 {
                    turn.get_score_diff()
                } else {
                    let mut scores: Vec<i16> = Vec::new();

                    for row in 0..board::BOARD_SIZE {
                        for col in 0..board::BOARD_SIZE {
                            let _ = turn.make_move(Coord::new(row, col))
                                .map( |turn_after_move| self.eval(&turn_after_move, depth -1))
                                .map( |new_score| scores.push(new_score) );
                        }
                    }

                    *match side {
                        reversi::Side::Dark  => scores.iter().min().unwrap(),
                        reversi::Side::Light => scores.iter().max().unwrap(),
                    }
                }
            }
        }
    }
}






/// Runs a full game using `FoolPlayer` for benchmarking purposes.
#[bench]
fn bench_game_fool(b: &mut test::Bencher) {
    b.iter(|| {
        let d: FoolPlayer = FoolPlayer;
        let l: FoolPlayer = FoolPlayer;
        let mut game: Game<(), FoolPlayer, FoolPlayer> = Game::new(&d, &l);

        while game.get_current_state().is_some() {
            game.play_turn().unwrap();
        }
    });
}

/// Runs a full game using `SimplePlayer` for benchmarking purposes.
#[bench]
fn bench_game_simple(b: &mut test::Bencher) {
    b.iter(|| {
        let d: SimplePlayer = SimplePlayer;
        let l: SimplePlayer = SimplePlayer;
        let mut game: Game<(), SimplePlayer, SimplePlayer> = Game::new(&d, &l);

        while game.get_current_state().is_some() {
            game.play_turn().unwrap();
        }
    });
}

/// Runs a full game using `FoolPlayer` and `SimplePlayer` for benchmarking purposes.
#[bench]
fn bench_game_fool_vs_simple(b: &mut test::Bencher) {
    b.iter(|| {
        let d: FoolPlayer = FoolPlayer;
        let l: SimplePlayer = SimplePlayer;
        let mut game: Game<(), FoolPlayer, SimplePlayer> = Game::new(&d, &l);

        while game.get_current_state().is_some() {
            game.play_turn().unwrap();
        }
    });
}
