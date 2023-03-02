

use array2d::Array2D;
use rand::Rng;

pub struct Game{
    pub board: Array2D<i64>,
    pub points: i64,
}

pub trait NormalGame{
    fn new() -> Self;
    fn initialize_board() -> Array2D<i64>;
}

impl Game{
    //returns board
    pub fn get_board(&self) ->  &Array2D<i64> {
        &self.board
    }

    pub fn get_mut_board(&mut self) -> &mut Array2D<i64> {
        &mut self.board
    }

    pub fn swipe_right(&mut self) {
        let board = self.get_mut_board();
        for i in 0..4 {
            let mut n = 3;
            while let Some(value) = Game::check_left(board, i, n - 1) {
                if n == 0{
                    break;
                }
                match board.get(i, n){
                    Some(0) => {
                        board[(i,n)] = value;
                        
                        //second element
                        if let Some(a) = Game::check_left(board, i, n-1){
                            if a == value{
                                board[(i,n)] = a + value;
                            }
                            else {
                                board[(i,n-1)] = a;
                            }
                            n -= 1;
                        }
                        else {
                            break;
                        }
                    },
                    Some(a) => {
                        if *a == value{
                            board[(i,n)] = a + value;
                        }
                        else{
                            board[(i,n-1)] = value;
                        }
                        n-=1;
                    },
                    None => (),
                }
            }
        }
    }

    fn check_left(board: &mut Array2D<i64>, x: usize, y: usize) -> Option<i64>{
        // y is horizontal, x is vertical
        match y{
            0 => match *board.get(x, y).unwrap(){
                0 => None, // return -1 because it has reached the end
                a => {
                    board.set(x, y, 0).unwrap();
                    Some(a)
                },
            },
            _ => match *board.get(x, y).unwrap() {
                0 => Game::check_left(board, x, y-1),
                b => {
                    board.set(x, y, 0).unwrap();
                    Some(b)
                }
            },
        }
    }

    pub fn print_board(&self){
        let board = self.get_board();
        for row_iter in board.rows_iter(){
            for element in row_iter{
                print!("{} ", element);
            }
            println!();
        }
        println!();
    }

}

impl NormalGame for Game{
    //Makes a new game with an empty board and 
    fn new() -> Game{
        Game{
            board: <Game as NormalGame>::initialize_board(), 
            points: 0,
        }
    }

    // initializes board with 2 in two random spots
    fn initialize_board() -> Array2D<i64> {
        let mut rng = rand::thread_rng();
        let (mut x1, mut x2, mut y1, mut y2) = (0, 0, 0, 0);
        //While loop makes sure the two initial values don't start in the same spot
        while x1 == x2 && y1 == y2 {
            (x1, x2, y1, y2) = (
                rng.gen_range(0..4),
                rng.gen_range(0..4),
                rng.gen_range(0..4),
                rng.gen_range(0..4)
            );
        } // generating random positions

        let mut board = Array2D::filled_with(0, 4, 4);
        board.set(x1, y1, 2)
            .unwrap();
        board.set(x2, y2, 2)
            .unwrap();
        board
    }
}

trait Swipable{
    fn test() -> Self;
}

