
///Contains the current board state
/// All the pieces, empty tiles etc.
#[derive(Copy, Clone)]
struct Board {
}

///Represents a move on the board. It can be analysed.
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
struct Move {

}

impl Board {
    ///Generates all possible moves on a provided board.
    ///Returns them as an array of moves.
    fn generate_moves(&self) -> Vec<Move> {
        vec![]
    }

    ///Applies a provided move to the board state, and returns the modified board.
    fn apply_move(&mut self, m: &Move) -> Board {
        Board {}
    }
}

///Represents a potential move that could be made, we just don't know if it is the best one yet.
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
struct PossibleMove {
    m: Move,
    c: i32,
}

///A recursive function that finds the best possible move when provided a board.
fn analyse( board: Board, r: i32, c: i32, o: Option<&Move> ) -> PossibleMove {
    let mut possible_moves: Vec<PossibleMove> = vec![];

    //We have reached the bottom of the tree, return a val!
    if r == 0 {
        return PossibleMove {
            m: o.unwrap().to_owned(), 
            c
        };
    }

    board.generate_moves().iter().for_each(|m| {
        let new_c = c;
        
        //Apply some cost function based on move m
        //Modify c here



        //This check is needed for the first loop of the iterative function as it won't have a valid move yet.
        let mut new_b = board.clone();
        if let Some(pot_m) = o {
            new_b = new_b.apply_move(pot_m);
        }

        possible_moves.push(analyse(new_b.clone().apply_move(m), r-1, new_c, Some(m)));
    });

    if possible_moves.is_empty() {
        //No possible moves, that's really bad.
        //Do something useful here instead like returning an empty move with negative infinity cost function.
    }

    possible_moves.sort();
    return possible_moves[0].clone();
}

fn main() {
    let mut my_board = Board{};
    let search_depth = 3;

    let my_move: PossibleMove = analyse(my_board, search_depth, 0, None);

    my_board.apply_move(&my_move.m);
}