///! Simple AIs to test the library's performances.

use board::*;
use turn::*;
use game::*;
use ::Result;
use rand;
use rand::{Rng};
use rand::distributions::{IndependentSample};

const RANDOMNESS: f64 = 0.05f64;

/// A stupid class implementing `IsPlayer` to be used for testing purposes.
pub struct FoolPlayer;

impl IsPlayer<()> for FoolPlayer {
    /// `FoolPlayer` plays a random (though legal) move.
    fn make_move(&self, turn: &Turn) -> Result<PlayerAction<()>> {
        let mut moves = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let coord = Coord::new(row, col);
                // println!("Fool checks {:?}", coord);
                if turn.check_move(coord).is_ok() {
                    if RANDOMNESS == 0f64 {
                        return Ok(PlayerAction::Move(coord));
                    } else {
                        moves.push(coord);
                    }
                }
            }
        }
        let chosen_move = rand::thread_rng().choose(&moves).ok_or(::ReversiError::EndedGame(*turn))?;
        Ok(PlayerAction::Move(*chosen_move))
    }
}

/// A simple class implementing `IsPlayer` to be used for testing purposes.
pub struct SimplePlayer;

impl SimplePlayer {

    #[inline(always)]
    fn is_better_dark(new_score: i16, old_score: i16) -> bool {
        new_score < old_score
    }

    #[inline(always)]
    fn is_better_light(new_score: i16, old_score: i16) -> bool {
        new_score > old_score
    }

    #[inline(always)]
    fn eval(&self, turn: &Turn, depth: u8) -> i16 {
        match turn.get_state() {
            None => turn.get_score_diff() * NUM_CELLS as i16,
            Some(side) => {
                if depth == 0 {
                    turn.get_score_diff()
                } else {
                    let is_better_than: fn(i16, i16) -> bool;
                    let mut best_score;
                    match side {
                        ::Side::Dark  => {
                            best_score = i16::max_value();
                            is_better_than = SimplePlayer::is_better_dark;
                        }
                        ::Side::Light => {
                            best_score = i16::min_value();
                            is_better_than = SimplePlayer::is_better_light;
                        }
                    }

                    let mut new_turn = turn.clone();

                    for row in 0..BOARD_SIZE {
                        for col in 0..BOARD_SIZE {
                            let coord = Coord::new(row, col);
                            if new_turn.make_move(coord).is_ok() {
                                let new_score = self.eval(&new_turn, depth - 1);
                                new_turn = turn.clone();
                                if is_better_than(new_score, best_score) {
                                    best_score = new_score;
                                }
                            }
                        }
                    }
                    best_score
                }
            }
        }
    }
}

impl IsPlayer<()> for SimplePlayer {

    fn make_move(&self, turn: &Turn) -> Result<PlayerAction<()>> {
        let is_better_than: fn(i16, i16) -> bool;
        let mut best_move = Coord::new(0, 0);
        if let Some(current_state_side) = turn.get_state() {
            let mut best_score;
            match current_state_side {
                ::Side::Dark  => {
                    best_score = i16::max_value();
                    is_better_than = SimplePlayer::is_better_dark;
                }
                ::Side::Light => {
                    best_score = i16::min_value();
                    is_better_than = SimplePlayer::is_better_light;
                }
            }

            let mut new_turn = turn.clone();
            let mut rng = rand::thread_rng();

            for row in 0..BOARD_SIZE {
                for col in 0..BOARD_SIZE {
                    let coord = Coord::new(row, col);
                    if new_turn.make_move(coord).is_ok() {
                        let new_score = ( self.eval(&new_turn, 3) as f64 * match RANDOMNESS > 0f64 {
                            false => 1f64,
                            true => {
                                let between = rand::distributions::Range::new(-RANDOMNESS, RANDOMNESS);
                                (1.0 + between.ind_sample(&mut rng))
                            }
                        } ) as i16;
                        new_turn = turn.clone();
                        if is_better_than(new_score, best_score) {
                            best_move = coord;
                            best_score = new_score;
                        }
                    }
                }
            }
            // println!("Simple plays {:?}", best_move);
            Ok(PlayerAction::Move(best_move))
        } else {
            Err(::ReversiError::EndedGame(*turn))
        }
    }
}
