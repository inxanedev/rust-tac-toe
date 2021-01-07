use std::io::{self, Write};
use std::fmt;

/// Represents individual piece in a game of tic-tac-toe.
#[derive(Copy, Clone, PartialEq)]
enum Piece {
    X,
    O,
    Empty,
}
impl Piece {
    /// Returns the opposite piece, used for taking turns
    fn opposite(&self) -> Self {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            Piece::Empty => Piece::Empty,
        }
    }
}
/// Function for printing a Piece onto the screen.
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Piece::X => "X",
            Piece::O => "O",
            Piece::Empty => " ",
        })
    }
}
/// Represents the result of a game.
#[derive(PartialEq)]
enum WinResult {
    Win,
    Draw,
    None,
}
/// Represents a Winner of a game.
/// The piece variable contains what piece has won.
/// The status represents the outcome of the game, can be Win or Draw.
struct Winner {
    piece: Piece,
    status: WinResult,
}
/// Represents a Board
/// The state is the 2D array of individual pieces.
/// The moves variable contains the amount of moves already played, used for detecting draws and the end of the game.
struct Board {
    state: [[Piece; 3]; 3],
    moves: u8,
}
/// Represents a corner in the tic-tac-toe board.
enum Corner {
    TopLeft,
    TopRight,
}

impl Board {
    /// Returns a new board with empty pieces
    fn new() -> Self {
        Self {
            state: [[Piece::Empty; 3]; 3],
            moves: 0,
        }
    }
    /// Sets the specified piece and returns true if the piece was not occupied
    fn set_piece(&mut self, x: usize, y: usize, piece: Piece) -> bool {
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
    fn check_winner(&mut self) -> Option<Winner> {
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
/// Represents a translation error when converting index to coordinate tuple.
struct TranslationError;
/// Converts the input number to a tuple of coordinates.
/// Returns a TranslationError if the input was invalid.
fn index_to_coordinates(index: usize) -> Result<(usize, usize), TranslationError> {
    if index >= 7 && index <= 9 {
        return Ok((index - 7, 0));
    } else if index >= 4 && index <= 6 {
        return Ok((index - 4, 1));
    } else if index >= 1 && index <= 3 {
        return Ok((index - 1, 2));
    } else {
        return Err(TranslationError);
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_piece = Piece::X;
    loop {
        println!("{}", board);
        loop {
            print!("Input move for {}: ", current_piece);
            io::stdout().flush().expect("Couldn't flush stdout!");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Couldn't read line from stdin!");
            let input: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid move (1 - 9)!");
                    continue;
                }
            };
            let (x, y) = match index_to_coordinates(input) {
                Ok(c) => c,
                Err(_) => {
                    println!("Please enter a valid move (1 - 9)!");
                    continue;
                }
            };
            if !board.set_piece(x, y, current_piece) {
                println!("That square is already occupied!");
                continue;
            } else {
                break;
            }
        }
        match board.check_winner() {
            Some(winner) => {
                if winner.status == WinResult::Win {
                    println!("{}", board);
                    println!("Player {} won!", winner.piece);
                } else {
                    println!("It's a draw!");
                }
                break;
            },
            None => {}
        }
        current_piece = current_piece.opposite();
    }
}