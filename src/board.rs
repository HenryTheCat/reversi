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
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {

    pub fn new(row: usize, col: usize) -> Coord {
        Coord {
            row: row,
            col: col,
        }
    }

    pub fn check_bounds(&self) -> Result<()> {
        if self.row >= BOARD_SIZE || self.col >= BOARD_SIZE {
            Err(::ReversiError::OutOfBoundCoord(*self))
        } else {
            Ok(())
        }
    }

    pub fn into_index(&self) -> Result<usize> {
        self.check_bounds().map(|()| self.row * BOARD_SIZE + self.col)
    }

    pub fn from_index(index: usize) -> Result<Coord> {
        if index < NUM_CELLS {
            Ok(Coord {
                row: index / BOARD_SIZE,
                col: index % BOARD_SIZE,
            })
        } else {
            Err(::ReversiError::OutOfBoundIndex(index))
        }
    }

    /// Produces new indexes moving along a direction and checking for out-of-bound errors (meaning sub-zero indexes).
    pub fn step(&mut self, dir: Direction) -> Result<()> {
        let (row, col) = (self.row, self.col);
        match dir {
            Direction::North    if row > 0              => Ok({self.row -= 1;}),
            Direction::NE       if row > 0              => Ok({self.row -= 1; self.col += 1;}),
            Direction::East                             => Ok({self.col += 1;}),
            Direction::SE                               => Ok({self.row += 1; self.col += 1;}),
            Direction::South                            => Ok({self.row += 1;}),
            Direction::SW       if col > 0              => Ok({self.row += 1; self.col -= 1;}),
            Direction::West     if col > 0              => Ok({self.col -= 1;}),
            Direction::NW       if row > 0 && col > 0   => Ok({self.row -= 1; self.col -= 1;}),
            _ => Err(::ReversiError::OutOfBoundStep(*self, dir)),
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
pub struct Board(pub [Cell; NUM_CELLS]);

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
    pub fn get_cell(&self, coord: Coord) -> Result<&Cell> {
        self.0.get(try!(coord.into_index())).ok_or(::ReversiError::OutOfBoundCoord(coord))
    }

    pub fn get_cell_by_index(&self, index: usize) -> Result<&Cell> {
        self.0.get(index).ok_or(::ReversiError::OutOfBoundIndex(index))
    }

    pub fn get_mut_cell(&mut self, coord: Coord) -> Result<&mut Cell> {
        self.0.get_mut(try!(coord.into_index())).ok_or(::ReversiError::OutOfBoundCoord(coord))
    }

    pub fn get_mut_cell_by_index(&mut self, index: usize) -> Result<&mut Cell> {
        self.0.get_mut(index).ok_or(::ReversiError::OutOfBoundIndex(index))
    }

    pub fn flip_disk(&mut self, coord: Coord) -> Result<()> {
        try!(self.get_mut_cell(coord)).map(|mut disk| disk.flip()).ok_or(::ReversiError::EmptyCell(coord))
    }

    pub fn flip_disk_by_index(&mut self, index: usize) -> Result<()> {
        try!(self.get_mut_cell_by_index(index)).map(|mut disk| disk.flip()).ok_or(::ReversiError::EmptyCell(try!(Coord::from_index(index))))
    }

    pub fn place_cell(&mut self, new_cell: Cell, coord: Coord) -> Result<()> {
        self.get_mut_cell(coord).map(|mut cell| *cell = new_cell)
    }

    pub fn place_cell_by_index(&mut self, new_cell: Cell, index: usize) -> Result<()> {
        self.get_mut_cell_by_index(index).map(|mut cell| *cell = new_cell)
    }

    pub fn place_disk(&mut self, disk: Disk, coord: Coord) -> Result<()> {
        self.get_mut_cell(coord).map(|mut cell| *cell = Some(disk))
    }

    pub fn place_disk_by_index(&mut self, disk: Disk, index: usize) -> Result<()> {
        self.get_mut_cell_by_index(index).map(|mut cell| *cell = Some(disk))
    }
}
