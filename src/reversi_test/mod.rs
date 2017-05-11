//! Test module.

mod test_ai;

use board::*;
use turn::*;
use game::*;
use test;
use reversi_test::test_ai::*;

/// Checks `turn::check_move` method on starting turn.
#[test]
fn test_first_turn() {
    let first_turn = Turn::first_turn();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let coord = Coord::new(row, col);
            assert!(first_turn.check_move(coord).is_ok() == match coord.get_row_col() {
                (2, 3) | (3, 2) | (4, 5) | (5, 4) => true,
                _ => false,
            }, "fails at {:?} because {:?}", coord, first_turn.check_move(coord))
        }
    }
}

#[test]
fn test_second_turn() {
    let mut second_turn = Turn::first_turn();
    second_turn.check_and_move(Coord::new(2, 3)).expect("Is this move illegal?");
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let coord = Coord::new(row, col);
            assert!(second_turn.check_move(coord).is_ok() == match coord.get_row_col() {
                (2, 2) | (2, 4) | (4, 2) => true,
                _ => false,
            }, "fails at {:?} because {:?}", coord, second_turn.check_move(coord))
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
