//! AI tests

#![feature(test)]

/// Simple AIs to test the library's performances.

extern crate rand;
extern crate test;
extern crate reversi;

use reversi::board::*;
use reversi::turn::*;
use reversi::game::*;
use reversi::{Result, ReversiError, Side};
use rand::{Rng};
use rand::distributions::{IndependentSample};
use std::cmp::Ordering;

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
                    moves.push(coord);
                }
            }
        }
        let chosen_move = rand::thread_rng().choose(&moves).ok_or_else(|| ReversiError::EndedGame(*turn))?;
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
                        Side::Dark  => {
                            best_score = i16::max_value();
                            is_better_than = SimplePlayer::is_better_dark;
                        }
                        Side::Light => {
                            best_score = i16::min_value();
                            is_better_than = SimplePlayer::is_better_light;
                        }
                    }

                    let mut new_turn = *turn;

                    for row in 0..BOARD_SIZE {
                        for col in 0..BOARD_SIZE {
                            let coord = Coord::new(row, col);
                            if new_turn.make_move(coord).is_ok() {
                                let new_score = self.eval(&new_turn, depth - 1);
                                new_turn = *turn;
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

            let mut new_turn = *turn;
            let mut rng = rand::thread_rng();
            let between = rand::distributions::Range::new(-RANDOMNESS, RANDOMNESS);

            for row in 0..BOARD_SIZE {
                for col in 0..BOARD_SIZE {
                    let coord = Coord::new(row, col);
                    if new_turn.make_move(coord).is_ok() {
                        let new_score = ( self.eval(&new_turn, 3) as f64 * (1.0 + between.ind_sample(&mut rng)) ) as i16;
                        new_turn = *turn;
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


/// Runs a full game using `FoolPlayer` for benchmarking purposes.
#[bench]
fn bench_game_fool(b: &mut test::Bencher) {
    let d: FoolPlayer = FoolPlayer;
    let l: FoolPlayer = FoolPlayer;
    let mut d_wins = 0;
    let mut l_wins = 0;
    let mut ties = 0;
    let mut d_total_score: u32 = 0;
    let mut l_total_score: u32 = 0;
    let mut game: Game<(), FoolPlayer, FoolPlayer> = Game::new(&d, &l);
    b.iter(|| {
        game = Game::new(&d, &l);
        while game.get_current_state().is_some() {
            game.play_turn().unwrap();
        }
        let (d_score, l_score) = game.get_current_score();
        d_total_score += d_score as u32;
        l_total_score += l_score as u32;
        match d_score.cmp(&l_score) {
	        Ordering::Greater => d_wins += 1,
	        Ordering::Less => l_wins += 1,
	        Ordering::Equal => ties += 1,
	    }
    });
    let total_matches = d_wins + l_wins + ties;
    println!("Outcome");
    println!("Dark  (Fool) wins {} match(es) with avarage score {}", d_wins, d_total_score as f32 / total_matches as f32);
    println!("Light (Fool) wins {} match(es) with avarage score {}", l_wins, l_total_score as f32 / total_matches as f32);
    println!("Tied match(es): {}", ties);
}

/// Runs a full game using `FoolPlayer` and `SimplePlayer` for benchmarking purposes.
#[bench]
fn bench_game_fool_vs_simple(b: &mut test::Bencher) {
    let d: FoolPlayer = FoolPlayer;
    let l: SimplePlayer = SimplePlayer;
    let mut d_wins = 0;
    let mut l_wins = 0;
    let mut ties = 0;
    let mut d_total_score: u32 = 0;
    let mut l_total_score: u32 = 0;
    let mut game: Game<(), FoolPlayer, SimplePlayer> = Game::new(&d, &l);
    b.iter(|| {
        game = Game::new(&d, &l);
        while game.get_current_state().is_some() {
            game.play_turn().unwrap();
        }
        let (d_score, l_score) = game.get_current_score();
        d_total_score += d_score as u32;
        l_total_score += l_score as u32;
        match d_score.cmp(&l_score) {
	        Ordering::Greater => d_wins += 1,
	        Ordering::Less => l_wins += 1,
	        Ordering::Equal => ties += 1,
	    }
    });
    let total_matches = d_wins + l_wins + ties;
    println!("Outcome");
    println!("Dark  (Fool)   wins {} match(es) with avarage score {}", d_wins, d_total_score as f32 / total_matches as f32);
    println!("Light (Simple) wins {} match(es) with avarage score {}", l_wins, l_total_score as f32 / total_matches as f32);
    println!("Tied match(es): {}", ties);
}

/// Runs a full game using `SimplePlayer` and `FoolPlayer` for benchmarking purposes.
#[bench]
fn bench_game_simple_vs_fool(b: &mut test::Bencher) {
    let d: SimplePlayer = SimplePlayer;
    let l: FoolPlayer = FoolPlayer;
    let mut d_wins = 0;
    let mut l_wins = 0;
    let mut ties = 0;
    let mut d_total_score: u32 = 0;
    let mut l_total_score: u32 = 0;
    let mut game: Game<(), SimplePlayer, FoolPlayer> = Game::new(&d, &l);
    b.iter(|| {
        game = Game::new(&d, &l);
        while game.get_current_state().is_some() {
            game.play_turn().unwrap();
        }
        let (d_score, l_score) = game.get_current_score();
        d_total_score += d_score as u32;
        l_total_score += l_score as u32;
        match d_score.cmp(&l_score) {
	        Ordering::Greater => d_wins += 1,
	        Ordering::Less => l_wins += 1,
	        Ordering::Equal => ties += 1,
	    }
    });
    let total_matches = d_wins + l_wins + ties;
    println!("Outcome");
    println!("Dark  (Simple)  wins {} match(es) with avarage score {}", d_wins, d_total_score as f32 / total_matches as f32);
    println!("Light (Fool)    wins {} match(es) with avarage score {}", l_wins, l_total_score as f32 / total_matches as f32);
    println!("Tied match(es): {}", ties);
}

/// Runs a full game using `SimplePlayer` for benchmarking purposes.
#[bench]
fn bench_game_simple(b: &mut test::Bencher) {
    let d: SimplePlayer = SimplePlayer;
    let l: SimplePlayer = SimplePlayer;
    let mut d_wins = 0;
    let mut l_wins = 0;
    let mut ties = 0;
    let mut d_total_score: u32 = 0;
    let mut l_total_score: u32 = 0;
    let mut game: Game<(), SimplePlayer, SimplePlayer> = Game::new(&d, &l);
    b.iter(|| {
        game = Game::new(&d, &l);
        while game.get_current_state().is_some() {
            game.play_turn().unwrap();
        }
        let (d_score, l_score) = game.get_current_score();
        d_total_score += d_score as u32;
        l_total_score += l_score as u32;
        match d_score.cmp(&l_score) {
	        Ordering::Greater => d_wins += 1,
	        Ordering::Less => l_wins += 1,
	        Ordering::Equal => ties += 1,
	    }
    });
    let total_matches = d_wins + l_wins + ties;
    println!("Outcome");
    println!("Dark  (Simple) wins {} match(es) with avarage score {}", d_wins, d_total_score as f32 / total_matches as f32);
    println!("Light (Simple) wins {} match(es) with avarage score {}", l_wins, l_total_score as f32 / total_matches as f32);
    println!("Tied match(es): {}", ties);
}
