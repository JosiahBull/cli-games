///This class represents the board state
use crate::ALPHABET;

extern crate termion;
use termion::{color, style};

#[derive(PartialEq)]
pub enum Tile {
    EmptySqaure,
    WhiteChecker,
    WhiteKing,
    BlackChecker,
    BlackKing
}

impl std::fmt::Display for Tile {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error>{
        match *self {
            Tile::EmptySqaure => fmt.write_str("*"),
            Tile::WhiteChecker => fmt.write_str("C"),
            Tile::WhiteKing => fmt.write_str("K"),
            Tile::BlackChecker => fmt.write_str("J"),
            Tile::BlackKing => fmt.write_str("L"),
        }
    }
}

pub struct Board {
    size: usize,
    //* = empty space, C == "white" checker, K == "white" king, J = "black" checker, L = "black" king, This is an internal representation only.
    board: Vec<Vec<Tile>>
}

impl Board {
    ///Create a new board
    pub fn new(size: usize) -> Self {
        //Generate the empty board (makes for easier printing later on)
        let mut board: Vec<Vec<Tile>> = vec![];
        for i in 0..size {
            let mut row: Vec<Tile> = vec![];
            for j in 0..size {
                row.push(Tile::EmptySqaure);
            }
            board.push(row);
        }

        Board {
            size,
            board,
        }
    }
    ///Add a piece to the board, takes a tuple representing the location of the piece
    pub fn add_piece(&mut self, piece: (usize, usize), p_type: Tile) {
        self.board[piece.0][piece.1] = p_type;
    }

    ///Print the board to the user
    pub fn print_board(&self) {
        //Print header row
        print!("   ");
        for i in 0..self.size {
            print!(" {} ", ALPHABET[(i as usize)]);
        }
        print!("\n\n");

        //Print main board
        for i in 0..self.size {
            print!("{}  ", ALPHABET[(i as usize)]);
            for j in 0..self.size {
                print!(" {} ", self.board[(i as usize)][(j as usize)]);
            }
            print!("\n");
        }
    }

    ///Check if the game is over
    pub fn check_over(&self) -> bool {
        return false;
    }

    ///Moves a piece on the board, takes two parameters. The piece to be moved, and where it is moving to.
    pub fn make_move(&mut self, piece: (usize, usize), new_loc: (usize, usize)) -> Result<(), String> {
        //Get the piece
        let piece_type: &Tile = &self.board[piece.0][piece.1];
        let new_loc_state: &Tile = &self.board[new_loc.0][new_loc.1];

        if new_loc.0 > self.size || new_loc.1 > self.size {
            return Err("Invalid location".into());
        }

        match piece_type {
            Tile::EmptySqaure => return Err("Not a piece".into()),
            Tile::BlackChecker => {
                //Black checkers move up, so we would expect them to be cool
            }
        }

        // if new_loc_state == Tile::EmptySqaure {
        //     //Empty Sqaure! Lets move there.
        //     self.board[piece.0][piece.1] = Tile::EmptySqaure;
        //     self.board[new_loc.0][new_loc.1] = piece_type;

        // } else {
        //     match piece_type {
                
        //     }
        // }
        Ok(())
    }
}