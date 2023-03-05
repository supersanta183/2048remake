use array2d::Array2D;
use rand::Rng;
use std::{thread,time};
use crate::direction;

use crate::direction::{Direction, DirectionValues, DirectionController};

pub struct Game {
    pub board: Array2D<i64>,
    pub inner_loop_counter: usize,
    pub outer_loop_counter: usize,
    pub cur_value: i64,
    pub points: i64,
}

pub trait NormalGame {
    fn new() -> Self;
    fn initialize_board() -> Array2D<i64>;
}

pub trait CustomGame {
    fn new() -> Self;
    fn initialize_board() -> Array2D<i64>;
    fn add_at_position(&mut self, element: i64, x: usize, y: usize);
}

impl Game {

    pub fn get_board(&self) -> &Array2D<i64>{
        &self.board
    }

    pub fn get_mutable_board(&mut self) -> &mut Array2D<i64> {
        &mut self.board
    }

    pub fn get_inner_loop_counter(&self) -> &usize {
        &self.inner_loop_counter
    }

    pub fn get_outer_loop_counter(&self) -> &usize {
        &&self.outer_loop_counter
    }

    pub fn set_inner_loop_counter(&mut self, a: usize){
        self.inner_loop_counter = a;
    }

    pub fn set_outer_loop_counter(&mut self, a: usize){
        self.outer_loop_counter = a;
    }

    pub fn swipe<T: Direction>(&mut self, dir: &DirectionController<T>) {
        for outer_loop in dir.get_outer_loop_range(){
            self.inner_loop_counter = dir.get_inner_loop_range();
            self.outer_loop_counter = outer_loop;
            self.execute_swipe(dir);
        }
    }

    fn execute_swipe<T: Direction>(&mut self, dir: &DirectionController<T>){
        while dir.evaluate_inner_loop(self){
            let cur_value = self.board.get(self.outer_loop_counter, self.inner_loop_counter);
            let cur_pos = (self.outer_loop_counter, self.inner_loop_counter);
            println!("Execute swipe cur pos : x : {}, y : {}", self.outer_loop_counter, self.inner_loop_counter);
            match cur_value {
                Some(value) => {
                    self.cur_value = *value;
                    if *value == 0{
                        self.current_value_is_0(cur_pos, dir);
                    }
                    else {
                        self.current_value_is_not_0(cur_pos, dir);
                    }
                },
                None => break,
            };
        }
    }

    fn current_value_is_0<T:Direction>(&mut self, (cur_pos_outer, cur_pos_inner): (usize, usize), dir: &DirectionController<T>){
        loop{
            println!(" 0 outer : {}, inner: {}",self.outer_loop_counter, self.inner_loop_counter);
            if dir.evaluate_row_loop(self){
                break;
            }

            println!("curx : {}, cury : {}", cur_pos_outer, cur_pos_inner);
            let temp = dir.get_next_value(self);
            let next_value;

            match temp {
                Some(a) => next_value = a,
                None => break,
            }

            //next value is not 0, pop that value and check if there is an equal value after that.
            if next_value != 0{
                println!("hej");
                dir.update_loop_counter(self);
                self.board[(cur_pos_outer,cur_pos_inner)] = self.pop_from_position((self.outer_loop_counter,self.inner_loop_counter));
                Game::print_board(&self.board);
                
                println!("hej 2");

                // number moved, check if next number is equal. Merge if they are.
                self.merge_next_number(next_value, cur_pos_outer, cur_pos_inner, dir);
                break;
            }
            else {
                //next value is 0, recursive call to check next value
                dir.update_loop_counter(self);
                self.current_value_is_0((cur_pos_outer,cur_pos_inner), dir);
                break;
            }
        }

    }

    fn current_value_is_not_0<T: Direction>(&mut self, (cur_pos_outer, cur_pos_inner): (usize, usize), dir: &DirectionController<T>){
        loop{
            let temp = dir.get_next_value(self);
            let next_value;

            match temp {
                Some(a) => next_value = a,
                None => break,
            }
            println!(" not 0 outer : {}, inner: {}",self.outer_loop_counter, self.inner_loop_counter);
            if dir.evaluate_row_loop(self){
                break;
            }
            println!("next : {}, cur value : {}", next_value, self.cur_value);
            if next_value != 0 && next_value == self.cur_value{
                dir.update_loop_counter(self);
                self.board[(cur_pos_outer,cur_pos_inner)] = self.pop_from_position((self.outer_loop_counter,self.inner_loop_counter)) + self.cur_value;
                break;
            }
            else {
                dir.update_loop_counter(self);
                break;
            }
        }
        Game::print_board(&self.board);
    }

    fn merge_next_number<T: Direction>(&mut self, next_value: i64, cur_pos_outer: usize, cur_pos_inner: usize, dir: &DirectionController<T>){
        println!("merge Current position: x : {}, y : {}", cur_pos_outer, cur_pos_inner);
        for _x in dir.get_merge_loop_range(self) {
            println!("{}", self.get_inner_loop_counter());
            let temp_value = dir.get_next_value(self);
            
            println!("out : {}, in: {}", cur_pos_outer, cur_pos_inner);

            match temp_value {
                Some(0) => break,
                Some(temp) => {
                    if temp == next_value{
                        println!("her");
                        self.board[(cur_pos_outer,cur_pos_inner)] = self.pop_from_position(dir.increment_value(self)) + temp;
                        Game::print_board(&self.board);
                        break;
                    }
                    else {
                        break;
                    }
                },
                None => break,
            }
        }
    }

    fn pop_from_position(&mut self, (x,y): (usize,usize)) -> i64{
        println!("pop position: x : {}, y : {}",x, y);
        let temp_value = self.board[(x,y)];
        self.board[(x,y)] = 0;
        temp_value
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

impl NormalGame for Game {
    //Makes a new game with an empty board and 
    fn new() -> Game {
        Game {
            board: <Game as NormalGame>::initialize_board(),
            inner_loop_counter: 0,
            outer_loop_counter: 0,
            cur_value: 0,
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


impl CustomGame for Game{
    fn new() -> Game{
        Game{
            board: <Game as CustomGame>::initialize_board(),
            inner_loop_counter: 0,
            outer_loop_counter: 0,
            cur_value: 0,
            points: 0,
        }
    }
    fn initialize_board() -> Array2D<i64> {
        Array2D::filled_with(0, 4, 4)
    }
    fn add_at_position(&mut self, element: i64, x: usize, y: usize){
        self.board.set(x, y, element).unwrap();
    }

}