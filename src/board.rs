//! Implementation of a 2D board (and of its constituing elements) with coordinates and iterators.

use std::fmt;
use ::Result;

/// The number of cells per side of the board.
pub const BOARD_SIZE: usize = 8;

/// The total number of cells of the board. Derived from `BOARD_SIZE` for ease of use.
pub const NUM_CELLS: usize = BOARD_SIZE * BOARD_SIZE;

/// Enums all the cardinal directions.
/// #Examples
/// If I am in cell `(4, 5)` and move `NE`, I go to cell `(3, 6)`.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    NE,
    East,
    SE,
    South,
    SW,
    West,
    NW,
}

/// Lists all cardinal directions from `Direction`.
pub const DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NE,
    Direction::East,
    Direction::SE,
    Direction::South,
    Direction::SW,
    Direction::West,
    Direction::NW
];

/// Coordinates of a cell, given by a row and a column.
/// Follows matrices conventions (see <https://en.wikipedia.org/wiki/Matrix_(mathematics)>) but for starting indexes at 0.
#[derive(Debug, Clone, Copy)]
pub struct Coord(usize, usize);

impl Coord {

    #[inline(always)]
    pub fn new(row: usize, col: usize) -> Coord {
        Coord(row, col)
    }

    #[inline(always)]
    pub fn get_row(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn get_col(&self) -> usize {
        self.1
    }

    #[inline(always)]
    pub fn get_row_col(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    /// Produces new indexes moving along a direction and checking for out-of-bound errors (meaning sub-zero indexes).
    #[inline(always)]
    pub fn step(&mut self, dir: Direction) {
        match dir {
            Direction::North    => self.0 = self.0.wrapping_sub(1),
            Direction::NE       => {
                self.0 = self.0.wrapping_sub(1);
                self.1 = self.1.wrapping_sub(1);
            },
            Direction::East     => self.1 = self.1.wrapping_sub(1),
            Direction::SE       => {
                self.0 += 1;
                self.1 = self.1.wrapping_sub(1);
            },
            Direction::South    => self.0 += 1,
            Direction::SW       => {
                self.0 += 1;
                self.1 += 1;
            },
            Direction::West     => self.1 += 1,
            Direction::NW       => self.0 = self.0.wrapping_sub(1),
        }
    }
}

/// A disk is characterized by its two sides, one Dark and one Light.
#[derive(Debug, Clone, Copy)]
pub struct Disk(::Side);

impl Disk {
    /// Creates a new disk with given side.
    #[inline(always)]
    pub fn new(side: ::Side) -> Disk {
        Disk(side)
    }

    /// Return's a disk's side.
    #[inline(always)]
    pub fn get_side(&self) -> ::Side {
        self.0
    }

    /// Turns the disk on the other side.
    #[inline(always)]
    pub fn flip(&mut self) {
        self.0 = self.0.opposite();
    }
}

/// Each cell in the board can either be empty or taken by one of the players.
pub type Cell = Option<Disk>;

#[derive(Copy)]
pub struct Board([[Cell; BOARD_SIZE]; BOARD_SIZE]);

impl fmt::Debug for Board {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This should be a board")
    }
}

impl Clone for Board {
    #[inline(always)]
    fn clone(&self) -> Board {
        *self
    }
}

/// `Board` is the type of boards, which are made by a `Frame`.
impl Board {

    #[inline(always)]
    pub fn new(board: [[Cell; BOARD_SIZE]; BOARD_SIZE]) -> Board {
        Board(board)
    }

    #[inline(always)]
    pub fn get_cell(&self, coord: Coord) -> Result<&Cell> {
        self.0.get(coord.get_row()).ok_or(::ReversiError::OutOfBoundCoord(coord))?
            .get(coord.get_col()).ok_or(::ReversiError::OutOfBoundCoord(coord))
    }

    #[inline(always)]
    fn get_mut_cell(&mut self, coord: Coord) -> Result<&mut Cell> {
        self.0.get_mut(coord.get_row()).ok_or(::ReversiError::OutOfBoundCoord(coord))?
            .get_mut(coord.get_col()).ok_or(::ReversiError::OutOfBoundCoord(coord))
    }

    #[inline(always)]
    pub fn flip_disk(&mut self, coord: Coord) -> Result<()> {
        self.get_mut_cell(coord).and_then(|mut cell| {
            cell.as_mut()
                .ok_or(::ReversiError::EmptyCell(coord))?
                .flip();
            Ok(())
        })
    }

    #[inline(always)]
    pub fn is_empty(&self, coord: Coord) -> Result<bool> {
        self.get_cell(coord).map(|&cell| cell.is_none())
    }

    #[inline(always)]
    pub fn place_disk(&mut self, side: ::Side, coord: Coord) -> Result<()> {
        self.get_mut_cell(coord).and_then(|mut cell| {
            if cell.is_some() {
                Err(::ReversiError::CellAlreadyTaken(coord))
            } else {
                *cell = Some(Disk::new(side));
                Ok(())
            }
        })
    }

    #[inline(always)]
    pub fn get_board(&self) -> &[[Cell; BOARD_SIZE]; BOARD_SIZE] {
        &self.0
    }

}
