//! Implementation of Reversi rules to play a turn.

use board::*;
use ::Result;

/// A turn can be in two states: either running (with a side to play next) or ended.
pub type State = Option<::Side>;

/// A turn is given by a board and by which player has to move next.
/// For convenience we also annotate current scores.
#[derive(Debug, Clone, Copy)]
pub struct Turn {
    board: Board,
    state: State,
    score_dark: u8,
    score_light: u8,
}

impl Turn {
    /// Initializing a new first turn: starting positions on the board and Dark is the first to play
    #[inline(always)]
    pub fn first_turn() -> Turn {
        let mut board = Board::new([[None; BOARD_SIZE]; BOARD_SIZE]);
        let center = BOARD_SIZE/2;
        board.place_disk(::Side::Dark, Coord::new(center - 1, center))
            .and(board.place_disk(::Side::Dark, Coord::new(center, center - 1)))
            .and(board.place_disk(::Side::Light, Coord::new(center - 1, center - 1)))
            .and(board.place_disk(::Side::Light, Coord::new(center, center)))
            .expect("Initial board setup failed");

        Turn {
            board: board,
            state: Some(::Side::Dark),
            score_dark: 2,
            score_light: 2,
        }
    }

    /// Returns the turn's board
    #[inline(always)]
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Returns the board's cell corresponding to the given coordinates.
    #[inline(always)]
    pub fn get_cell(&self, coord: Coord) -> Result<&Cell> {
        self.board.get_cell(coord)
    }

    /// Returns the turn's status
    #[inline(always)]
    pub fn get_state(&self) -> State {
        self.state
    }

    /// Returns whether the turn has `None` state.
    #[inline(always)]
    pub fn is_end_state(&self) -> bool {
        self.state.is_none()
    }

    /// Returns the current score of the match.
    #[inline(always)]
    pub fn get_score(&self) -> (u8, u8) {
        (self.score_dark, self.score_light)
    }

    /// Returns the difference in score between Light and Dark.
    #[inline(always)]
    pub fn get_score_diff(&self) -> i16 {
        self.score_light as i16 - self.score_dark as i16
    }

    /// Returns turn's tempo (how many disks there are on the board).
    #[inline(always)]
    pub fn get_tempo(&self) -> u8 {
        self.score_light + self.score_dark
    }

    /// Checks whether a move leads to eat in a specified direction
    #[inline(always)]
    fn check_move_along_direction (&self, coord: Coord, dir: Direction, side: ::Side) -> bool {
        let mut next_coord = coord.step(dir);
        if let Ok(&Some(next_disk)) = self.get_cell(next_coord) {
            if next_disk.get_side() != side {
                next_coord = next_coord.step(dir);
                while let Ok(&Some(next_disk)) = self.board.get_cell(next_coord) {
                    if next_disk.get_side() == side {
                        return true;
                    }
                    next_coord = next_coord.step(dir);
                }
            }
        }
        false
    }

    /// Check whether a given move is legal
    #[inline(always)]
    pub fn check_move (&self, coord: Coord) -> Result<()> {
        // If the game is ended, no further moves are possible
        let state_side = self.state.ok_or_else(|| ::ReversiError::EndedGame(*self))?;
        const THIRD_TO_LAST: usize = BOARD_SIZE - 3;
        const SECOND_TO_LAST: usize = BOARD_SIZE - 2;

        macro_rules! check_move_along_directions {
            ($dir:ident, $($dirs:ident),+) => (
                self.check_move_along_direction(coord, Direction::$dir, state_side) || check_move_along_directions!($($dirs),+)
            );
            ($dir:ident) => (
                self.check_move_along_direction(coord, Direction::$dir, state_side)
            );
        }

        if self.board.get_cell(coord)?.is_some() {
            // If a cell is already taken, it's not possible to move there
            Err(::ReversiError::CellAlreadyTaken(coord))
        } else if match coord.get_row() {
            0...1 => check_move_along_directions!(South)
                || (coord.get_col() >= 2 && check_move_along_directions!(West, SW))
                || (coord.get_col() < SECOND_TO_LAST && check_move_along_directions!(East, SE)),
            2...THIRD_TO_LAST => check_move_along_directions!(North, South)
                || (coord.get_col() >= 2 && check_move_along_directions!(West, SW, NW))
                || (coord.get_col() < SECOND_TO_LAST && check_move_along_directions!(East, NE, SE)),
            _ => check_move_along_directions!(North)
                || (coord.get_col() >= 2 && check_move_along_directions!(West, NW))
                || (coord.get_col() < SECOND_TO_LAST && check_move_along_directions!(East, NE)),
        } {
            // If a move leads to eat in at least one direction, then it is legal
            Ok(())
        } else {
            // Otherwise, the move is not legal
            Err(::ReversiError::IllegalMove(coord))
        }
    }

    /// Current player performs a move, after verifying that it is legal.
    /// It returns either the new turn or the error preventing the move to be performed.
    #[inline(always)]
    pub fn make_move (&mut self, coord: Coord) -> Result<()> {
        if self.get_cell(coord)?.is_none() {
            let turn_side = self.state.ok_or_else(|| ::ReversiError::EndedGame(*self))?;
            let mut legal = false;
            let mut eating: u8 = 0;
            for &dir in DIRECTIONS.into_iter().filter(|&&dir|
                (coord.get_row() >= 2                   || (dir != Direction::North && dir != Direction::NE && dir != Direction::NW))
                && (coord.get_row() < BOARD_SIZE - 2    || (dir != Direction::South && dir != Direction::SE && dir != Direction::SW))
                && (coord.get_col() >= 2                || (dir != Direction::West  && dir != Direction::NW && dir != Direction::SW))
                && (coord.get_col() < BOARD_SIZE - 2    || (dir != Direction::East  && dir != Direction::NE && dir != Direction::SE)) ) {
                if self.check_move_along_direction(coord, dir, turn_side) {
                    // Eats all of the opponent's occupied cells from a specified cell (given by its coordinates) in a specified direction until it finds a cell of the current player.
                    let mut next_coord = coord.step(dir);
                    self.board.flip_disk(next_coord)
                        .expect("Eating in this direction has already been checked to work!");
                    eating += 1;
                    next_coord = next_coord.step(dir);
                    while let Ok(&Some(disk)) = self.board.get_cell(next_coord) {
                        if disk.get_side() != turn_side {
                            self.board.flip_disk(next_coord)
                                .expect("Eating in this direction has already been checked to work!");
                            eating += 1;
                            next_coord = next_coord.step(dir);
                        } else {
                            legal = true;
                            break;
                        }
                    }
                }
            }
            if legal {
                self.board.place_disk(turn_side, coord)
                    .expect("This cell has already been checked empty!");
                match turn_side {
                    ::Side::Dark => {
                        self.score_light -= eating;
                        self.score_dark  += eating + 1;
                    }
                    ::Side::Light => {
                        self.score_light += eating + 1;
                        self.score_dark  -= eating;
                    }
                }
                // If a move is legal, the next player to play has to be determined
                // If the opposite player can make any move at all, it gets the turn
                // If not, if the previous player can make any move at all, it gets the turn
                // If not (that is, if no player can make any move at all) the game is ended
                if self.get_tempo() == NUM_CELLS as u8 {
                    // Quick check to rule out games with filled up boards as ended.
                    self.state = None;
                } else {
                    // Turn passes to the other player.
                    self.state = Some(turn_side.opposite());
                    if !self.can_move() {
                        // If the other player cannot move, turn passes back to the first player.
                        self.state = Some(turn_side);
                        if !self.can_move() {
                            // If neither platers can move, game is ended.
                            self.state = None;
                        }
                    }
                }
                Ok(())
            } else {
                Err(::ReversiError::IllegalMove(coord))
            }
        } else {
            Err(::ReversiError::CellAlreadyTaken(coord))
        }
    }

    /// Returns whether or not next_player can make any move at all.
    /// To be used privately. User should rather look at turn's state.
 //     #[inline(always)]
 //     fn can_move(&self) -> bool {
 //        (0..BOARD_SIZE).any(|row|
 //            (0..BOARD_SIZE).any(|col|
 //                self.check_move(Coord::new(row, col)).is_ok()))
 //     }


    #[inline(always)]
    fn can_move(&self) -> bool {
        if let Some(state_side) = self.state {
            macro_rules! check_move_along_directions {
                ($coord:expr, $($dirs:ident),+) => ({
                    let coord = $coord;
                    self.get_cell(coord).expect("This coord should be alright!").is_none()
                        && check_move_along_directions_after_setup!(coord, $($dirs),+)
                });
            }
            macro_rules! check_move_along_directions_after_setup {
                ($coord:ident, $dir:ident, $($dirs:ident),+) => (
                    self.check_move_along_direction($coord, Direction::$dir, state_side)
                        || check_move_along_directions_after_setup!($coord, $($dirs),+)
                );
                ($coord:ident, $dir:ident) => (
                    self.check_move_along_direction($coord, Direction::$dir, state_side)
                );
            }

            (BOARD_SIZE-2..BOARD_SIZE).any(|row| {
                (BOARD_SIZE-2..BOARD_SIZE).any(|col| check_move_along_directions!(Coord::new(row, col), North, West, NW))
                || (2..BOARD_SIZE-2).any(|col| check_move_along_directions!(Coord::new(row, col), North, NE, East, West, NW))
                || (0..2).any(|col| check_move_along_directions!(Coord::new(row, col), North, NE, East))
            } ) || (2..BOARD_SIZE-2).any(|row| {
                (BOARD_SIZE-2..BOARD_SIZE).any(|col| check_move_along_directions!(Coord::new(row, col), North, West, South, SW, NW))
                || (2..BOARD_SIZE-2).any(|col| check_move_along_directions!(Coord::new(row, col), North, NE, East, SE, South, SW, West, NW))
                || (0..2).any(|col| check_move_along_directions!(Coord::new(row, col), North, NE, East, SE, South))
            } ) || (0..2).any(|row| {
                (BOARD_SIZE-2..BOARD_SIZE).any(|col| check_move_along_directions!(Coord::new(row, col), West, South, SW))
                || (2..BOARD_SIZE-2).any(|col| check_move_along_directions!(Coord::new(row, col), East, SE, South, SW, West))
                || (0..2).any(|col| check_move_along_directions!(Coord::new(row, col), East, SE, South))
            })
        } else {
            false
        }
    }

}
