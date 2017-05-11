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

    pub fn new(row: usize, col: usize) -> Coord {
        Coord(row, col)
    }

    pub fn get_row(&self) -> usize {
        self.0
    }

    pub fn get_col(&self) -> usize {
        self.1
    }

    pub fn get_row_col(&self) -> (usize, usize) {
        (self.get_row(), self.get_col())
    }

    /// Produces new indexes moving along a direction and checking for out-of-bound errors (meaning sub-zero indexes).
    pub fn step(&mut self, dir: Direction) {
        match dir {
            Direction::North    => self.0 = self.0.wrapping_sub(1), //self.step_north(),
            Direction::NE       => {self.0 = self.0.wrapping_sub(1); self.1 = self.1.wrapping_sub(1);},// self.step_north().and(self.step_east()),
            Direction::East     => self.1 = self.1.wrapping_sub(1), // self.step_east(),
            Direction::SE       => {self.0 += 1; self.1 = self.1.wrapping_sub(1);}, // self.step_south().and(self.step_east()),
            Direction::South    => self.0 += 1, // self.step_south(),
            Direction::SW       => {self.0 += 1; self.1 += 1;}, // self.step_south().and(self.step_west()),
            Direction::West     => self.1 += 1, // self.step_west(),
            Direction::NW       => self.0 = self.0.wrapping_sub(1),// self.step_north().and(self.step_west()),
        }
    }
}

/// A disk is characterized by its two sides, one Dark and one Light.
#[derive(Debug, Clone, Copy)]
pub struct Disk(::Side);

impl Disk {
    /// Creates a new disk with given side.
    pub fn new(side: ::Side) -> Disk {
        Disk(side)
    }

    /// Return's a disk's side.
    pub fn get_side(&self) -> ::Side {
        self.0
    }

    /// Turns the disk on the other side.
    pub fn flip(&mut self) {
        self.0 = self.0.opposite();
    }
}

/// Each cell in the board can either be empty or taken by one of the players.
pub type Cell = Option<Disk>;

#[derive(Copy)]
pub struct Board([[Cell; BOARD_SIZE]; BOARD_SIZE]);

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This should be a board")
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        *self
    }
}

/// `Board` is the type of boards, which are made by a `Frame`.
impl Board {

    pub fn new(board: [[Cell; BOARD_SIZE]; BOARD_SIZE]) -> Board {
        Board(board)
    }

    pub fn get_cell(&self, coord: Coord) -> Result<&Cell> {
        let row = try!(self.0.get(coord.get_row()).ok_or(::ReversiError::OutOfBoundCoord(coord))); //.map(|&row| row.get(coord.get_col())) //.ok_or(::ReversiError::OutOfBoundCoord(coord))
        row.get(coord.get_col()).ok_or(::ReversiError::OutOfBoundCoord(coord))
    }

    pub fn get_board(&self) -> &[[Cell; BOARD_SIZE]; BOARD_SIZE] {
        &self.0
    }

    fn get_mut_cell(&mut self, coord: Coord) -> Result<&mut Cell> {
        let mut row = try!(self.0.get_mut(coord.get_row()).ok_or(::ReversiError::OutOfBoundCoord(coord))); //.map(|&row| row.get(coord.get_col())) //.ok_or(::ReversiError::OutOfBoundCoord(coord))
        row.get_mut(coord.get_col()).ok_or(::ReversiError::OutOfBoundCoord(coord))
    }

    pub fn flip_disk(&mut self, coord: Coord) -> Result<()> {
        try!(self.get_mut_cell(coord)).map(|mut disk| disk.flip()).ok_or(::ReversiError::EmptyCell(coord))
    }

    pub fn is_empty(&self, coord: Coord) -> Result<bool> {
        self.get_cell(coord).map(|&cell| cell.is_none())
    }

    pub fn place_disk(&mut self, disk: Disk, coord: Coord) -> Result<()> {
        self.get_mut_cell(coord).map(|mut cell| *cell = Some(disk))
    }

}
