use std::fmt;

/// Represents individual piece in a game of tic-tac-toe.
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    X,
    O,
    Empty,
}

impl Piece {
    /// Returns the opposite piece, used for taking turns
    pub fn opposite(&self) -> Self {
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