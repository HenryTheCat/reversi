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
    pub fn first_turn() -> Turn {
        let mut board = Board::new([[None; BOARD_SIZE]; BOARD_SIZE]);
        board.place_disk(Disk::new(::Side::Dark), Coord::new(3, 4)).expect("Initial board setup failed");
        board.place_disk(Disk::new(::Side::Dark), Coord::new(4, 3)).expect("Initial board setup failed");
        board.place_disk(Disk::new(::Side::Light), Coord::new(3, 3)).expect("Initial board setup failed");
        board.place_disk(Disk::new(::Side::Light), Coord::new(4, 4)).expect("Initial board setup failed");

        Turn {
            board: board,
            state: Some(::Side::Dark),
            score_dark: 2,
            score_light: 2,
        }
    }

    /// Returns the turn's board
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Returns the board's cell corresponding to the given coordinates.
    pub fn get_cell(&self, coord: Coord) -> Result<&Cell> {
        self.board.get_cell(coord)
    }

    /// Returns the turn's status
    pub fn get_state(&self) -> &State {
        &self.state
    }

    /// Returns the current score of the match.
    pub fn get_score(&self) -> (u8, u8) {
        (self.score_dark, self.score_light)
    }

    /// Returns the difference in score between Light and Dark.
    pub fn get_score_diff(&self) -> i16 {
        self.score_light as i16 - self.score_dark as i16
    }

    /// Returns turn's tempo (how many disks there are on the board).
    pub fn get_tempo(&self) -> u8 {
        self.score_light + self.score_dark
    }

    /// Check whether a given move is legal
    pub fn check_move (&self, coord: Coord) -> Result<()> {

        if self.state.is_none() {
            // If the game is ended, no further moves are possible
            Err(::ReversiError::EndedGame(*self))
        //} else if coord.check_bounds().is_err() {
        //    // If `coord` is out-of-bounds, the move is not legal.
        //    Err(::ReversiError::OutOfBoundCoord(coord))
        } else if try!(self.board.get_cell(coord)).is_some() {
            // If a cell is already taken, it's not possible to move there
            Err(::ReversiError::CellAlreadyTaken(coord))
        } else {
            // If a move leads to eat in at least one direction, then it is legal
            for &dir in &DIRECTIONS {
                if self.check_move_along_direction(coord, dir) {
                    return Ok(());
                }
            }
            // Otherwise, the move is not legal
            Err(::ReversiError::IllegalMove(coord))
        }
    }

    /// Checks whether a move leads to eat in a specified direction
    fn check_move_along_direction (&self, coord: Coord, dir: Direction) -> bool {
        if let Some(state_side) = self.state {
            let mut next_coord = coord;
            next_coord.step(dir);
            if let Ok(&next_cell) = self.get_cell(next_coord) {
                if let Some(next_disk) = next_cell {
                    if next_disk.get_side() == state_side.opposite() {
                        next_coord.step(dir);
                        while let Ok(&next_cell) = self.board.get_cell(next_coord) {
                            return match next_cell {
                                None => false,
                                Some(successive_disk) if successive_disk.get_side() == state_side => true,
                                _ => {next_coord.step(dir); continue},
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Eats all of the opponent's occupied cells from a specified cell (given by its coordinates) in a specified direction until it finds a cell of the current player.
    fn move_along_direction (&mut self, coord: Coord, dir: Direction) -> Result<()> {
        let turn = try!(self.state.ok_or(::ReversiError::EndedGame(*self)));
        let mut next_coord = coord;
        next_coord.step(dir);
        try!(self.board.place_disk(Disk::new(turn), next_coord));
        let mut eating: u8 = 1;

        next_coord.step(dir);
        while let Ok(&cell) = self.board.get_cell(next_coord) {
            match cell {
                Some(disk) => {
                    if disk.get_side() != turn {
                        try!(self.board.place_disk(Disk::new(turn), next_coord));
                        next_coord.step(dir);
                        eating += 1;
                    } else {
                        break;
                    }
                }
                None => return Err(::ReversiError::IllegalMove(coord)),
            }
        }
        match turn {
            ::Side::Light => {
                self.score_light += eating;
                self.score_dark  -= eating;
            }
            ::Side::Dark => {
                self.score_light -= eating;
                self.score_dark  += eating;
            }
        }
        Ok(())
    }

    ///// Checks whether a move leads to eat in a specified direction and then performs the move in the given direction.
    //fn check_and_move_along_direction (&mut self, coord: Coord, dir: Direction) -> Result<()> {
    //    if self.check_move_along_direction(coord, dir) {
    //        self.move_along_direction(coord, dir)
    //    } else {
    //        Err(::ReversiError::IllegalMove(coord))
    //    }
    //}

    /// Current player performs a move, after verifying that it is legal.
    /// It returns either the new turn or the error preventing the move to be performed.
    pub fn check_and_move (&mut self, coord: Coord) -> Result<()> {

        if let None = *try!(self.board.get_cell(coord)) {
            let mut legal = false;

            if let Some(turn_side) = self.state {
                for &dir in &DIRECTIONS {
                    if self.check_move_along_direction(coord, dir) {
                        self.move_along_direction(coord, dir).expect("The Move has already been checked!");
                        legal = true;
                    }
                }

                if legal {
                    try!(self.board.place_disk(Disk::new(turn_side), coord));
                    match turn_side {
                        ::Side::Dark  => self.score_dark  += 1,
                        ::Side::Light => self.score_light += 1,
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
                Err(::ReversiError::EndedGame(*self))
            }
        } else {
            Err(::ReversiError::CellAlreadyTaken(coord))
        }
    }

    /// Returns whether or not next_player can make any move at all.
    /// To be used privately. User should rather look at turn's state.
    fn can_move(&self) -> bool {
        //for row in 0..BOARD_SIZE {
        //    for col in 0..BOARD_SIZE {
        //if self.board.is_empty(coord).expect("Is the coordinate wrong?") {

        for (row, row_array) in self.board.get_board().iter().enumerate() {
            for (col, &cell) in row_array.iter().enumerate() {
                if cell.is_none() {
                    let coord = Coord::new(row, col);
                    for &dir in &DIRECTIONS {
                        if self.check_move_along_direction(coord, dir) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

}
