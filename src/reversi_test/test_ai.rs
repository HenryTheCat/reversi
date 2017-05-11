///! Simple AIs to test the library's performances.

use board::*;
use turn::*;
use game::*;
use ::Result;

/// A stupid class implementing `IsPlayer` to be used for testing purposes.
pub struct FoolPlayer;

impl IsPlayer<()> for FoolPlayer {
    fn make_move(&self, turn: &Turn) -> Result<PlayerAction<()>> {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let coord = Coord::new(row, col);
                // println!("Fool checks {:?}", coord);
                if turn.check_move(coord).is_ok() {
                    // println!("Fool plays {:?}", coord);
                    return Ok(PlayerAction::Move(coord));
                }
            }
        }
        println!("No move");
        Err(::ReversiError::EndedGame(*turn))
    }
}

/// A simple class implementing `IsPlayer` to be used for testing purposes.
pub struct SimplePlayer;

impl IsPlayer<()> for SimplePlayer {
    fn make_move(&self, turn: &Turn) -> Result<PlayerAction<()>> {
        let mut best_move = Coord::new(0, 0);
        let mut best_score = match *turn.get_state() {
            Some(::Side::Dark)  => i16::max_value(),
            Some(::Side::Light) => i16::min_value(),
            None => return Err(::ReversiError::EndedGame(*turn)),
        };

        let mut new_turn = turn;

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let coord = Coord::new(row, col);
                if new_turn.check_move(coord).is_ok() {
                    let new_score = self.eval(&new_turn, 3);
                    new_turn = turn;
                    match *turn.get_state() {
                        Some(::Side::Dark)  => {
                            if new_score < best_score {
                                best_move = coord;
                                best_score = new_score;
                            }
                        },
                        Some(::Side::Light) => {
                            if new_score > best_score {
                                best_move = coord;
                                best_score = new_score;
                            }
                        },
                        None => return Err(::ReversiError::EndedGame(*turn)),
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
        match *turn.get_state() {
            None => turn.get_score_diff() * NUM_CELLS as i16,
            Some(side) => {
                if depth == 0 {
                    turn.get_score_diff()
                } else {

                    let mut best_score = match side {
                        ::Side::Dark  => i16::max_value(),
                        ::Side::Light => i16::min_value(),
                    };

                    let mut new_turn = turn.clone();

                    for row in 0..BOARD_SIZE {
                        for col in 0..BOARD_SIZE {
                            let coord = Coord::new(row, col);
                            if new_turn.check_and_move(coord).is_ok() {
                                let new_score = self.eval(&new_turn, depth - 1);
                                new_turn = turn.clone();
                                match side {
                                    ::Side::Dark  => {
                                        if new_score < best_score {
                                            best_score = new_score;
                                        }
                                    },
                                    ::Side::Light => {
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
