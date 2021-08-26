///This class represents the board state
use crate::{ALPHABET, MAX_MOVES};

extern crate termion;

///Represents a sqaure on the board, and all the possible states it could be in.
#[derive(PartialEq, Clone, Copy)]
pub enum Tile {
    EmptySqaure,
    WhiteChecker,
    WhiteKing,
    BlackChecker,
    BlackKing
}

///Represents who's turn it is to move next.
pub enum PlayerMove {
    White,
    Black
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

///A struct which contains data about the game.
pub struct Board {
    ///Size of the board
    size: usize,
    //* = empty space, C == "white" checker, K == "white" king, J = "black" checker, L = "black" king, This is an internal representation only.
    ///The board state
    board: Vec<Vec<Tile>>,
    ///Who moves next
    to_move: PlayerMove,
    ///The number of total moves made
    moves: usize
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
            to_move: PlayerMove::White,
            moves: 0,
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
        if self.moves >= MAX_MOVES {
            return true;
        }
        let mut black_pieces = 0;
        let mut white_pieces = 0;
        for _i in self.board.iter() {
            for j in _i.iter() {
                match j {
                    Tile::BlackChecker | Tile::BlackKing => {
                        black_pieces += 1;
                    },
                    Tile::WhiteChecker | Tile::WhiteKing => {
                        white_pieces += 1;
                    },
                    _ => {}
                }
            }
        }
        if black_pieces == 0 || white_pieces == 0 {
            return true;
        }
        return false;
    }

    ///Moves a piece on the board, takes two parameters. The piece to be moved, and where it is moving to.
    pub fn make_move(&mut self, piece: (i32, i32), mut new_loc: (i32, i32)) -> Result<(), String> {
        //Get the piece
        let mut piece_taken: bool = false;
        let piece_type: Tile = self.board[piece.0 as usize][piece.1 as usize];
        let new_loc_state: Tile = self.board[new_loc.0 as usize][new_loc.1 as usize];

        if new_loc.0 > self.size as i32 || new_loc.1 > self.size as i32 {
            return Err("Invalid location".into());
        }
        //Check that the movement is allowed
        match piece_type {
            Tile::EmptySqaure => return Err("not a piece".into()),
            Tile::BlackChecker => {
                //Black checkers move up 
                //Check that their location is valid, should be diagonal. I.e. y increase by 1, x change by 1
                if !(new_loc.0 - piece.0 == -1 && (new_loc.1- piece.1).abs() == 1) {
                    return Err("Invalid Move Location (1)".into());
                }
            },
            Tile::WhiteChecker => {
                //White checkers move down
                //Check that their location is valid, should be diagonal. I.e. y decrease by 1, x change by 1
                if !((new_loc.0 - piece.0) == 1 && (new_loc.1 - piece.1).abs() == 1) {
                    return Err("Invalid Move Location (2)".into());
                }
            },
            Tile::BlackKing | Tile::WhiteKing => {
                if !((new_loc.0 - piece.0).abs() == 1 && (new_loc.1- piece.1).abs() == 1) {
                    return Err("Invalid Move Location (3)".into());
                }
            }
        };

        // Check where the piece is moving to, and then move it there.
        match (new_loc_state, piece_type) {
            (Tile::EmptySqaure,  _) => {
                //Great, it's an empty sqaure!
                self.board[new_loc.0 as usize][new_loc.1 as usize] = piece_type.to_owned();
                self.board[piece.0 as usize][piece.1 as usize] = Tile::EmptySqaure;
            },
            (Tile::BlackChecker | Tile::BlackKing, Tile::WhiteChecker | Tile::WhiteKing) | (Tile::WhiteChecker | Tile::WhiteKing, Tile::BlackChecker | Tile::BlackKing) => {
                //One Piece is going to take another!
                
                //Get the space behind the piece we are taking
                let new_loc_taken: (i32, i32) = (new_loc.0 + (new_loc.0 - piece.0), new_loc.1 + (new_loc.1 - piece.1));

                //Check we haven't gone beyond board bounds
                if new_loc_taken.0 >= self.size as i32 || new_loc_taken.1 >= self.size as i32 {
                    return Err("Unable to take piece: Not enough space behind it!".into());
                }

                //Check that the tile is empty
                if self.board[new_loc_taken.0 as usize][new_loc_taken.1 as usize] != Tile::EmptySqaure {
                    return Err("Tile after the piece you are trying to take is not empty!".into());
                }

                //Take the piece
                piece_taken = true;
                self.board[new_loc_taken.0 as usize][new_loc_taken.1 as usize] = piece_type.to_owned(); //Move to new location
                self.board[new_loc.0 as usize][new_loc.1 as usize] = Tile::EmptySqaure; //Set piece we took as an empty square
                self.board[piece.0 as usize][piece.1 as usize] = Tile::EmptySqaure; //Set our previous location as empty
                new_loc = new_loc_taken;
            },
            (_, _) => {
                //Any other movement is invalid (i.e. white taking white piece)
                return Err("Invalid Move".into());
            }
        }

        //Check to see if we are at the end of the board, to king the piece.
        match (piece_type, new_loc.0) {
            (Tile::BlackChecker, 0) => {
                self.board[new_loc.0 as usize][new_loc.1 as usize] = Tile::BlackKing; 
            },
            (Tile::WhiteChecker, size) if size == self.size as i32 -1 => {
                self.board[new_loc.0 as usize][new_loc.1 as usize] = Tile::WhiteKing; 
            },
            (_, _) => {/*Do nothing*/}
        }

        //Flip who is to move (if no piece was taken)
        if !piece_taken {
            match self.to_move {
                PlayerMove::White => self.to_move = PlayerMove::Black,
                PlayerMove::Black => self.to_move = PlayerMove::White
            }
        }

        self.moves += 1;
        Ok(())
    }
}