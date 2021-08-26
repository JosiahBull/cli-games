//! Welcome to Bex's Checkers Game
//! The main.rs represents the controller, documentation for other classes can be found below.
use std::io;

mod board;
use board::*;
///Represents the alphabet the game operates over, Obviously this limits the max size of the board to 26.
const ALPHABET: &[char] = &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
///The maximum number of moves to be made in a game before we declare it as being finished.
const MAX_MOVES: usize = 40;

///Convert a letter from the user into an integer in the alphabet (A == 0, B == 1, etc)
fn convert_letter_to_int(letter: &char) -> Result<i32, String> {
    for (i, cha) in ALPHABET.iter().enumerate() {
        if cha == letter {
            return Ok(i as i32);
        }
    }
    Err("char not recognised".into())
}

///Request a move from the user, and then convert that into a tile selection object.
fn get_move(msg: &str) -> Result<TileSelection, String> {
    println!("{}", msg);
    let mut piece = String::new();
    io::stdin().read_line(&mut piece).expect("Unable to read io stream to get input");
    if piece.len() < 2 {
        return Err("Not enough chars entered!".into());
    }
    let first_letter = convert_letter_to_int(&piece.chars().nth(0).unwrap())?;
    let second_letter = convert_letter_to_int(&piece.chars().nth(1).unwrap())?;

    Ok(TileSelection::new(first_letter, second_letter))
}

fn main() {
    //Create the board and pieces
    let mut board: Board = Board::new(4);
    board.add_piece(TileSelection::new(0,0), Tile::WhiteChecker);
    board.add_piece(TileSelection::new(0,2), Tile::WhiteChecker);
    board.add_piece(TileSelection::new(3,1), Tile::BlackChecker);
    board.add_piece(TileSelection::new(3,3), Tile::BlackChecker);
    println!("===========");
    println!("Welcome to Bex's Cool Checker Game");
    println!("===========");

    //Loop until game is over
    while !board.check_over() {
        board.print_board();
        //Get the piece they want to move
        let piece: TileSelection = match get_move(&format!("{}, select the piece you would like to move e.g. 'AA': ", board.get_move())) {
            //Don't worry too much about the code in here, it's just error handling
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let new_loc: TileSelection = match get_move(&format!("Select where {}{} should go: ", ALPHABET[piece.get_x() as usize], ALPHABET[piece.get_y() as usize])) {
            //Don't worry too much about the code in here, it's just error handling
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        //Attempt to make the move
        if let Err(message) = board.make_move(piece, new_loc) {
            //If the move was invalid, then tell them
            println!("Error! Try again. Reason: {}", message);
            continue;
        }

    }
    board.print_board();
    println!("Congratulations! Thank you for playing my dude!")
}