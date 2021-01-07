use std::fmt;

#[path = "piece.rs"]
pub mod piece;
use piece::*;


/// Represents a Board
/// The state is the 2D array of individual pieces.
/// The moves variable contains the amount of moves already played, used for detecting draws and the end of the game.
pub struct Board {
    pub state: [[Piece; 3]; 3],
    pub moves: u8,
}

impl Board {
    /// Returns a new board with empty pieces
    pub fn new() -> Self {
        Self {
            state: [[Piece::Empty; 3]; 3],
            moves: 0,
        }
    }
    /// Sets the specified piece and returns true if the piece was not occupied
    pub fn set_piece(&mut self, x: usize, y: usize, piece: Piece) -> bool {
        if self.state[y][x] == Piece::Empty {
            self.state[y][x] = piece;
            self.moves += 1;
            return true;
        } else {
            return false;
        }
    }
    /// Utility function to check three pieces at once
    fn check_three(a: Piece, b: Piece, c: Piece) -> WinResult {
        if a == b && b == c && a != Piece::Empty {
            WinResult::Win
        } else {
            WinResult::None
        }
    }
    /// Checks the specified row for win
    fn check_row(&mut self, row: usize) -> Winner {
        Winner {
            piece: self.state[row][0],
            status: Board::check_three(self.state[row][0], self.state[row][1], self.state[row][2])
        }
    }
    /// Checks the specified column for win
    fn check_column(&mut self, column: usize) -> Winner {
        Winner {
            piece: self.state[0][column],
            status: Board::check_three(self.state[0][column], self.state[1][column], self.state[2][column])
        }
    }
    /// Checks the specified diagonal for win
    fn check_diagonal(&mut self, corner: Corner) -> Winner {
        Winner {
            piece: match corner {
                Corner::TopLeft => self.state[0][0],
                Corner::TopRight => self.state[0][2]
            },
            status: match corner {
                Corner::TopLeft => {
                    Board::check_three(self.state[0][0], self.state[1][1], self.state[2][2])
                },
                Corner::TopRight => {
                    Board::check_three(self.state[0][2], self.state[1][1], self.state[2][0])
                },
            }
        }
    }
    /// Returns either the Winner or None if the game is still progressing.
    /// The Winner enum has a status, and that status can be a Draw if there's no more moves available, and no winner.
    pub fn check_winner(&mut self) -> Option<Winner> {
        let mut cur: Winner;
        // Rows
        for i in 0..3 {
            cur = self.check_row(i);
            if cur.status == WinResult::Win {
                return Some(cur);
            }
        };

        // Columns
        for i in 0..3 {
            cur = self.check_column(i);
            if cur.status == WinResult::Win {
                return Some(cur);
            }
        };

        // Diagonals
        cur = self.check_diagonal(Corner::TopLeft);
        if cur.status == WinResult::Win { return Some(cur); };
        cur = self.check_diagonal(Corner::TopRight);
        if cur.status == WinResult::Win { return Some(cur); };

        if self.moves == 9 {
            Some(Winner {
                piece: Piece::Empty,
                status: WinResult::Draw,
            });
        }
        None
    }
}
/// Display function for printing the board to the screen
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} | {} | {}\n-----------\n {} | {} | {}\n-----------\n {} | {} | {}",
               self.state[0][0], self.state[0][1], self.state[0][2],
               self.state[1][0], self.state[1][1], self.state[1][2],
               self.state[2][0], self.state[2][1], self.state[2][2]
        )
    }
}

/// Represents the result of a game.
#[derive(PartialEq)]
pub enum WinResult {
    Win,
    Draw,
    None,
}
/// Represents a Winner of a game.
/// The piece variable contains what piece has won.
/// The status represents the outcome of the game, can be Win or Draw.
pub struct Winner {
    pub piece: Piece,
    pub status: WinResult,
}

/// Represents a corner in the tic-tac-toe board.
enum Corner {
    TopLeft,
    TopRight,
}