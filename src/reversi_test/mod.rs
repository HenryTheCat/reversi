//! Test module.

mod test_ai;

use test;
use board::*;
use turn::*;
use game::*;
use reversi_test::test_ai::*;
use std::cmp::Ordering;

/// Checks `turn::check_move` method on starting turn.
#[bench]
fn test_first_turn(b: &mut test::Bencher) {
    let first_turn = Turn::first_turn();
    b.iter(|| {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let coord = Coord::new(row, col);
                assert!(first_turn.check_move(coord).is_ok() == match coord.get_row_col() {
                    (2, 3) | (3, 2) | (4, 5) | (5, 4) => true,
                    _ => false,
                }, "fails at {:?} because {:?}", coord, first_turn.check_move(coord))
            }
        }
    });
}

#[bench]
fn test_second_turn(b: &mut test::Bencher) {
    let mut second_turn = Turn::first_turn();
    second_turn.make_move(Coord::new(2, 3)).expect("Is this move illegal?");
    b.iter(|| {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let coord = Coord::new(row, col);
                assert!(second_turn.check_move(coord).is_ok() == match coord.get_row_col() {
                    (2, 2) | (2, 4) | (4, 2) => true,
                    _ => false,
                }, "fails at {:?} because {:?}", coord, second_turn.check_move(coord))
            }
        }
    });
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
