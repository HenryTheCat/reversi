//! Test module.

// #![feature(test)]
// extern crate test;
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
                // println!("Fool checks {:?}", coord);
                if turn.check_move(coord).is_ok() {
                    // println!("Fool plays {:?}", coord);
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
                if turn.check_move(coord).is_ok() {
                    let new_score = self.eval(&turn.make_move(coord).unwrap(), 3);
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
        // println!("Simple plays {:?}", best_move);
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

                    let mut best_score = match side {
                        Side::Dark  => i16::max_value(),
                        Side::Light => i16::min_value(),
                    };

                    for row in 0..board::BOARD_SIZE {
                        for col in 0..board::BOARD_SIZE {
                            let coord = Coord::new(row, col);
                            if turn.check_move(coord).is_ok() {
                                let new_score = self.eval(&turn.make_move(coord).unwrap(), depth - 1);
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
                    }
                    best_score
                }
            }
        }
    }
}

#[test]
fn test_board() {
    let mut board = board::Board::new(&[[None; board::BOARD_SIZE]; board::BOARD_SIZE]);
    board.place_disk(Side::Dark, Coord::new(0, 0));
    assert!(board.get_cell(Coord::new(0, 0)).unwrap().unwrap().get_side() == Side::Dark);
    board.flip_disk(Coord::new(0, 0));
    assert!(board.get_cell(Coord::new(0, 0)).unwrap().unwrap().get_side() == Side::Light);
}


/// Checks `turn::check_move` method on starting turn.
#[test]
fn test_first_turn() {
    let first_turn = Turn::first_turn();

    for row in 0..board::BOARD_SIZE {
        for col in 0..board::BOARD_SIZE {
            let coord = Coord::new(row, col);
            assert!(first_turn.check_move(coord).is_ok() == match coord.get_row_col() {
                (2, 3) | (3, 2) | (4, 5) | (5, 4) => true,
                _ => false,
            }, "fails at {:?} because {:?}", coord, first_turn.check_move(coord))
        }
    }

    let second_turn = first_turn.make_move(Coord::new(2, 3)).unwrap();
    for row in 0..board::BOARD_SIZE {
        for col in 0..board::BOARD_SIZE {
            let coord = Coord::new(row, col);
            assert!(second_turn.check_move(coord).is_ok() == match coord.get_row_col() {
                (2, 2) | (2, 4) | (4, 2) => true,
                _ => false,
            }, "fails at {:?} because {:?}", coord, second_turn.check_move(coord))
        }
    }

}

/// Runs a full game using `FoolPlayer` for benchmarking purposes.
#[test]
fn bench_game_fool() {
    let d: FoolPlayer = FoolPlayer;
    let l: FoolPlayer = FoolPlayer;
    let mut game: Game<(), FoolPlayer, FoolPlayer> = Game::new(&d, &l);

    while game.get_current_state().is_some() {
        game.play_turn().unwrap();
    }
}

/// Runs a full game using `SimplePlayer` for benchmarking purposes.
#[test]
fn bench_game_simple() {
    let d: SimplePlayer = SimplePlayer;
    let l: SimplePlayer = SimplePlayer;
    let mut game: Game<(), SimplePlayer, SimplePlayer> = Game::new(&d, &l);

    while game.get_current_state().is_some() {
        game.play_turn().unwrap();
    }
}

/// Runs a full game using `FoolPlayer` and `SimplePlayer` for benchmarking purposes.
#[test]
fn bench_game_fool_vs_simple() {
    let d: FoolPlayer = FoolPlayer;
    let l: SimplePlayer = SimplePlayer;
    let mut game: Game<(), FoolPlayer, SimplePlayer> = Game::new(&d, &l);

    while game.get_current_state().is_some() {
        game.play_turn().unwrap();
    }
}
