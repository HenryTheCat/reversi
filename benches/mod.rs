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
        for index in 0..board::NUM_CELLS {
            let coord = Coord::from_index(index).unwrap();
            // println!("Fool checks {:?}", coord);
            if turn.check_move(coord).is_ok() {
                // println!("Fool plays {:?}", coord);
                return Ok(PlayerAction::Move(coord));
            }
        }
        println!("No move");
        Err(ReversiError::EndedGame(*turn))
    }
}

/// A simple class implementing `IsPlayer` to be used for testing purposes.
struct SimplePlayer;

impl game::IsPlayer<()> for SimplePlayer {
    fn make_move(&self, turn: &Turn) -> reversi::Result<PlayerAction<()>> {
        let mut best_move = Coord{row: 0, col: 0};
        let mut best_score = match *turn.get_state() {
            Some(Side::Dark)  => i16::max_value(),
            Some(Side::Light) => i16::min_value(),
            None => return Err(ReversiError::EndedGame(*turn)),
        };

        for index in 0..board::NUM_CELLS {
            let coord = Coord::from_index(index).unwrap();
            if turn.check_move(coord).is_ok() {
                let new_score = self.eval(&turn.check_and_move(coord).unwrap(), 3);
                match *turn.get_state() {
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
                    None => return Err(ReversiError::EndedGame(*turn)),
                };
            }
        }
        // println!("Simple plays {:?}", best_move);
        Ok(PlayerAction::Move(best_move))
    }
}

impl SimplePlayer {
    fn eval(&self, turn: &Turn, depth: u8) -> i16 {
        match *turn.get_state() {
            None => turn.get_score_diff() * board::NUM_CELLS as i16,
            Some(side) => {
                if depth == 0 {
                    turn.get_score_diff()
                } else {

                    let mut best_score = match side {
                        Side::Dark  => i16::max_value(),
                        Side::Light => i16::min_value(),
                    };

                    for index in 0..board::NUM_CELLS {
                        let coord = Coord::from_index(index).unwrap();
                        if turn.check_move(coord).is_ok() {
                            let new_score = self.eval(&turn.check_and_move(coord).unwrap(), depth - 1);
                            match side {
                                Side::Dark  => {
                                    if new_score < best_score {
                                        best_score = new_score;
                                    }
                                },
                                Side::Light => {
                                    if new_score > best_score {
                                        best_score = new_score;
                                    }
                                },
                            };
                        }
                    }
                    best_score
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
