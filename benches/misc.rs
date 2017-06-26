//! Misc benches

#![feature(test)]

extern crate test;
extern crate reversi;

use reversi::board::*;
use reversi::turn::*;

/// Checks `turn::check_move` method on starting turn.
#[bench]
fn test_first_turn(b: &mut test::Bencher) {
    let first_turn = Turn::first_turn();
    b.iter(|| {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let coord = Coord::new(row, col);
                assert_eq!(first_turn.check_move(coord).is_ok(), match coord.get_row_col() {
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
                assert_eq!(second_turn.check_move(coord).is_ok(), match coord.get_row_col() {
                    (2, 2) | (2, 4) | (4, 2) => true,
                    _ => false,
                }, "fails at {:?} because {:?}", coord, second_turn.check_move(coord))
            }
        }
    });
}
