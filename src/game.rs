

use array2d::Array2D;
use rand::Rng;

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

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

    pub fn swipe(&mut self) {
        println!("swipe");
        let board = self.get_mut_board();
        for i in 0..4 {
            let mut n = 3;

            // gets next Number to the left, returns None if no more numbers
            while let Some(value) = Game::get_value_from(board, i, n - 1) {

                // no more columns to check
                println!("n : {}", n);
                if n == 0{
                    break;
                }
                println!("n : {}", n);

                // match value from current position on the board
                match board[(i,n)]{
                    //current position is 0, set to value
                    0 => {
                        board[(i,n)] = value;
                        
                        //checks if there is an equal value as the next number.
                        // Otherwise places next number directly in the next position
                        if let Some(a) = Game::get_value_from(board, i, n-1){
                            println!("foerst");
                            Game::set_board_value(board, i, n, a, value);
                            n -= 1;
                        }
                        else {
                            break;
                        }
                    },
                    // current position is not 0, update if equal to value
                    //Otherwise update next position.
                    a => {
                        Game::set_board_value(board, i, n, value, a);
                        println!("anden");
                        n-=1;
                    },
                }
            }
        }
    }

    fn set_board_value(board: &mut Array2D<i64>, i: usize, n: usize, a: i64, value: i64){
        if a == value{
            board[(i,n)] = a + value;
        }
        else {
            board[(i,n-1)] = a;
        }
    }


    fn get_value_from(board: &mut Array2D<i64>, x: usize, y: usize) -> Option<i64>{
        // y is horizontal, x is vertical
        println!("get value");

        match board[(x,y)] {
            0 => {
                if y == 0{
                    return None;
                }
                    Game::get_value_from(board, x, y-1)
            },
            a => {
                println!("x : {}, y : {}, val: {} ", x, y, board[(x,y)]);
                board[(x,y)] = 0;
                Some(a)
            },
        }
    }

    pub fn print_board(board: &Array2D<i64>){
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

