use std::iter::Enumerate;

use array2d::Array2D;
use rand::{Rng, seq::SliceRandom};

use crate::direction::{Direction, DirectionController};

pub struct Game {
    pub board: Array2D<i64>,
    pub inner_loop_counter: usize,
    pub outer_loop_counter: usize,
    pub cur_value: i64,
    pub points: i64,
    pub has_lost: bool,
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

    pub fn get_loser_status(&self) -> bool{
        self.has_lost
    }

    pub fn get_board(&self) -> &Array2D<i64> {
        &self.board
    }

    pub fn get_points(&self) -> &i64 {
        &self.points
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
        match self.check_valid_moves() {
            true => (),
            false => self.has_lost = true,
        }
        for outer_loop in dir.get_outer_loop_range(){
            dir.setup_loop_values(self, outer_loop);
            self.execute_swipe(dir);
        }
        self.generate_new_number();
    }

    fn execute_swipe<T: Direction>(&mut self, dir: &DirectionController<T>){
        while dir.evaluate_inner_loop(self){
            if dir.evaluate_row_loop(self){
                break;
            }
            let cur_value = self.board.get(self.outer_loop_counter, self.inner_loop_counter);
            let cur_pos = (self.outer_loop_counter, self.inner_loop_counter);
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
            if dir.evaluate_row_loop(self){
                break;
            }
            let temp = dir.get_next_value(self);
            let next_value;
            match temp {
                Some(a) => next_value = a,
                None => break,
            }

            //next value is not 0, pop that value and check if there is an equal value after that.
            if next_value != 0{
                self.cur_value = next_value;
                dir.update_loop_counter(self);
                let value = self.pop_from_position((self.outer_loop_counter,self.inner_loop_counter));
                self.push_at_position((cur_pos_outer,cur_pos_inner), value);
                // number moved, check if next number is equal. Merge if they are.
                self.merge_next_number(cur_pos_outer, cur_pos_inner, dir);
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
            if dir.evaluate_row_loop(self){
                break;
            }
            let temp = dir.get_next_value(self);
            let mut next_value;
            match temp {
                Some(a) => next_value = a,
                None => break,
            }
            
            // next value is 0 update loop counter
            if next_value == 0{
                dir.update_loop_counter(self);
            }
            // next value is equal to the current value, merge them
            else if next_value == self.cur_value{
                dir.update_loop_counter(self);
                let value = self.pop_from_position((*self.get_outer_loop_counter(),*self.get_inner_loop_counter())) + self.cur_value;
                self.push_at_position((cur_pos_outer,cur_pos_inner), value);
                self.update_points(value);
                break;
            }
            // next value is different from current value, put it in the next position
            else {
                dir.update_loop_counter(self);
                let value = self.pop_from_position((*self.get_outer_loop_counter(), *self.get_inner_loop_counter()));
                let (x,y) = dir.increment_value((cur_pos_outer, cur_pos_inner));
                self.push_at_position((x,y), value);
                break;
            }
        }
    }

    fn merge_next_number<T: Direction>(&mut self, cur_pos_outer: usize, cur_pos_inner: usize, dir: &DirectionController<T>){
        for _x in dir.get_merge_loop_range(self) {
            let temp = dir.get_next_value(self);
            let mut next_value;
            match temp {
                Some(a) => next_value = a,
                None => break,
            }

            //next value is 0, update loop counter
            if next_value == 0{
                dir.update_loop_counter(self);
            }
            // next value is equal, merge
            else if next_value == self.cur_value {
                let value = self.pop_from_position(dir.increment_value((*self.get_outer_loop_counter(), *self.get_inner_loop_counter()))) + next_value;
                self.push_at_position((cur_pos_outer,cur_pos_inner), value);
                self.update_points(value);
                break;
            }
            //next value is different from current value, put it in next position
            else {
                dir.update_loop_counter(self);
                let value = self.pop_from_position((*self.get_outer_loop_counter(), *self.get_inner_loop_counter()));
                let (x,y) = dir.increment_value((cur_pos_outer, cur_pos_inner));
                self.push_at_position((x,y), value);
                break;
            }
        }
    }

    fn push_at_position(&mut self, (xpos,ypos): (usize, usize), value: i64){
        self.get_mutable_board()[(xpos,ypos)] = value;
    }

    fn pop_from_position(&mut self, (x,y): (usize,usize)) -> i64{
        let temp_value = self.board[(x,y)];
        self.board[(x,y)] = 0;
        temp_value
    }

    fn update_points(&mut self, amount: i64){
        self.points = self.points + amount;
    }

    fn generate_new_number(&mut self){
        let mut rng = rand::thread_rng();
        let vec = vec![2,4];
        let num = vec.choose(&mut rng);
        let mut free_positions: Vec<(usize,usize)> = Vec::new();

        for i in 0..4{
            for n in 0..4{
                if self.board[(i,n)] == 0{
                    free_positions.push((i,n));
                }
            }
        }
        if !free_positions.is_empty(){
            let pos = free_positions.choose(&mut rng);
            self.push_at_position(*pos.unwrap(), *num.unwrap());
        }
    }

    fn check_valid_moves(&self) -> bool {
        let board = self.get_board();
        for i in 0..4 {
            for j in 0..4 {
                let val = board[(i,j)];
                match val {
                    0 => return true,
                    _ if j > 0 && val == board[(i,j-1)] => return true,
                    _ if i > 0 && val ==  board[(i-1,j)] => return true,
                    _ if j < 3 && val ==  board[(i,j+1)] => return true,
                    _ if i < 3 && val ==  board[(i+1,j)] => return true,
                    _ => (),
                };
            }
        }
        false
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
            has_lost: false,
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
            has_lost: false,
        }
    }
    fn initialize_board() -> Array2D<i64> {
        Array2D::filled_with(0, 4, 4)
    }
    fn add_at_position(&mut self, element: i64, x: usize, y: usize){
        self.board.set(x, y, element).unwrap();
    }

}