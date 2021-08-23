///This class is the game controller, which implements our main loop
use std::io::{self, Read};

mod board;
use board::*;

pub const ALPHABET: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

fn convert_letter_to_int(letter: &char) -> usize {
    for (i, cha) in ALPHABET.iter().enumerate() {
        if cha == letter {
            return i;
        }
    }
    panic!("Failed to find letter, something screwy was entered!");
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
        let mut piece = String::new();
        let mut moved = String::new();
        println!("Select the piece you would like to move e.g. 'AA': ");
        io::stdin().read_line(&mut piece).expect("Failed to parse input as string");
        println!("Select where the piece {} should go: ", piece.strip_suffix("\n").unwrap());
        io::stdin().read_line(&mut moved).expect("Failed to parse input as string");
        board.make_move((convert_letter_to_int(&piece.chars().nth(0).unwrap()), convert_letter_to_int(&piece.chars().nth(1).unwrap())), (convert_letter_to_int(&moved.chars().nth(0).unwrap()), convert_letter_to_int(&moved.chars().nth(1).unwrap())));
    }
}