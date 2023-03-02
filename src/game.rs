
use array2d::Array2D;
use rand::Rng;

use crate::direction::{Direction, DirectionValues};


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

    pub fn swipe(&mut self, direc: DirectionValues) {
        let dir = Direction::new(direc);
        for i in 0..4 {
            let n = dir.get_n();

            // gets next Number to the left, returns None if no more numbers
            dir.swipe_direction(self, i, n);
        }
    }

    pub fn swipe_content(&mut self, i:usize, n:usize, dir: &Direction) -> Option<(usize,usize)>{
        let x = self.get_value_from(i, n, dir);
        // no more columns to check
        if x.is_none(){
            return None;
        }
        let board = self.get_mut_board();
        let value = x.unwrap();
        println!("value {}", value);

        // match value from current position on the board
        match board[(i,n)]{
            //current position is 0, set to value
            0 => {
                println!("board 0");
                board[(i,n)] = value;
                //checks if there is an equal value as the next number.
                // Otherwise places next number directly in the next position
                let (x,y) = dir.increment_direction(i, n);
                if let Some(a) = self.get_value_from(x, y, dir){
                    println!("value {}", a);
                    self.set_board_value(i, n, a, value, dir);
                    Some(dir.increment_direction(i, n))
                }
                else {
                    None
                }
            },
            // current position is not 0, update if equal to value
            //Otherwise update next position.
            a => {
                print!("board a");
                self.set_board_value(i, n, value, a, dir);
                Some(dir.increment_direction(i, n))
            },
        }
    }

    fn set_board_value(&mut self, i: usize, n: usize, a: i64, value: i64, dir: &Direction){
        let board = self.get_mut_board();
        if a == value{
            board[(i,n)] = a + value;
        }
        else {
            board[(dir.increment_direction(i, n))] = a;
        }
    }


    fn get_value_from(&mut self, x: usize, y: usize, dir: &Direction) -> Option<i64>{
        // y is horizontal, x is vertical
        let board = self.get_mut_board();
        println!("x {}, y: {}",x, y);
        match board[(x,y)] {
            0 => {
                if let Some(l) = dir.get_value_comparison(self, x, y){
                    Some(l)
                }
                else {
                    None
                }
            },
            a => {
                board[(x,y)] = 0;
                Some(a)
            },
        }
    }

    pub fn get_value_recursive(&mut self, x: usize, y: usize, dir: &Direction) -> Option<i64>{
        let (i,n) = dir.increment_direction(x, y);
        self.get_value_from(i, n, dir)
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

