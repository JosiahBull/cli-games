///This class is the game controller, which implements our main loop
use std::io::{self, Read};

use colored::*;

mod board;
use board::*;

const ALPHABET: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
const MAX_MOVES: usize = 40;

fn convert_letter_to_int(letter: &char) -> Result<i32, String> {
    for (i, cha) in ALPHABET.iter().enumerate() {
        if cha == letter {
            return Ok(i as i32);
        }
    }
    Err("char not recognised".into())
}

fn get_move(msg: &str) -> Result<(i32, i32), String> {
    println!("{}", msg);
    let mut piece = String::new();
    io::stdin().read_line(&mut piece).expect("Unable to read io stream to get input");
    if &piece.len() < &2 {
        return Err("Not enough chars entered!".into());
    }
    Ok((convert_letter_to_int(&piece.chars().nth(0).unwrap())?, convert_letter_to_int(&piece.chars().nth(1).unwrap())?))
}

fn main() {
    //Create the board and pieces
    let mut board: Board = Board::new(4);
    board.add_piece((0,0), Tile::WhiteChecker);
    board.add_piece((0,2), Tile::WhiteChecker);
    board.add_piece((3,1), Tile::BlackChecker);
    board.add_piece((3,3), Tile::BlackChecker);
    println!("===========");
    println!("Welcome to Bex's Cool Checker Game");
    println!("===========");

    while !board.check_over() {
        board.print_board();
        let piece: (i32, i32) = match get_move("Select the piece you would like to move e.g. 'AA': ") {
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let new_loc: (i32, i32) = match get_move("Select where the piece should go: ") {
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        if let Err(message) = board.make_move(piece, new_loc) {
            println!("Wrong move! Try again. Reason: {}", message);
            continue;
        }

    }
    board.print_board();
    println!("Congratulations! Thank you for playing my dude!")
}