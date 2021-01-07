use std::io::{self, Write};

mod board;
use board::*;
use piece::*;

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