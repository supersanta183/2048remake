use crate::{game::Game};



const LEFT_N: usize = 0;
const UP_N: usize = 0;
const DOWN_N: usize = 3;
const RIGHT_N: usize = 3;


pub enum DirectionValues {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

pub trait Direction{
    fn create_outer_loop_range(&self) -> Vec<usize>;
    fn create_inner_loop_range(&self) -> usize;
    fn create_merge_loop_range(&self, game: &Game) -> Vec<usize>;
    fn update_loop_counter(&self, game: &mut Game);
    fn get_next_value(&self, game: &mut Game) -> Option<i64>;
    fn evaluate_inner_loop(&self, game: &Game) -> bool;
    fn evaluate_row_loop(&self, game: &Game) -> bool;
    fn increment_value(&self, (x,y): (usize, usize)) -> (usize,usize){(x,y)}
    fn setup_loop_values(&self, game: &mut Game, outer_loop: usize);
}

pub struct DirectionController<T: Direction>{
    direction_strategy: T,
}

impl<T: Direction> DirectionController<T>{
    pub fn new(direction_strat: T) -> Self{
        Self{ direction_strategy: direction_strat }
    }

    pub fn get_outer_loop_range(&self) -> Vec<usize> {
        self.direction_strategy.create_outer_loop_range()
    }

    pub fn get_inner_loop_range(&self) -> usize {
        self.direction_strategy.create_inner_loop_range()
    }

    pub fn get_merge_loop_range(&self, game: &Game) -> Vec<usize> {
        self.direction_strategy.create_merge_loop_range(game)
    }

    pub fn update_loop_counter(&self, game: &mut Game) {
        self.direction_strategy.update_loop_counter(game);
    }

    pub fn get_next_value(&self, game: &mut Game) -> Option<i64> {
        self.direction_strategy.get_next_value(game)
    }

    pub fn evaluate_inner_loop(&self, game: &Game) -> bool {
        self.direction_strategy.evaluate_inner_loop(game)
    }

    pub fn evaluate_row_loop(&self, game: &mut Game) -> bool {
        self.direction_strategy.evaluate_row_loop(game)
    }

    pub fn increment_value(&self, (x,y) : (usize, usize)) -> (usize, usize) {
        self.direction_strategy.increment_value((x,y))
    }

    pub fn setup_loop_values(&self, game: &mut Game, outer_loop: usize){
        self.direction_strategy.setup_loop_values(game, outer_loop);
    }
}

pub struct RightDirectionStrategy;

impl Direction for RightDirectionStrategy {
    fn create_outer_loop_range(&self) -> Vec<usize>{
        let mut vec = Vec::with_capacity(4);
        for i in 0..4{
            vec.push(i);
        }
        vec
    }

    fn create_inner_loop_range(&self) -> usize {
        RIGHT_N
    }

    fn update_loop_counter(&self, game: &mut Game) {
        let cur_count = game.get_inner_loop_counter();
        game.set_inner_loop_counter(cur_count-1);
    }

    fn get_next_value(&self, game: &mut Game) -> Option<i64> {
        let (x,y) = (*game.get_outer_loop_counter(),*game.get_inner_loop_counter());
        match game.get_mutable_board().get(x, y-1) {
            Some(a) => Some(*a),
            None => None
        }
    }

    fn evaluate_inner_loop(&self, game: &Game) -> bool {
        *game.get_inner_loop_counter() > 0
    }

    fn evaluate_row_loop(&self, game: &Game) -> bool {
        *game.get_inner_loop_counter() == 0
    }

    fn create_merge_loop_range(&self, game: &Game) -> Vec<usize> {
        let count = *game.get_inner_loop_counter();
        let mut vec = Vec::with_capacity(count);
        for i in (0..count).rev(){
            vec.push(i);
        }
        vec
    }

    fn increment_value(&self, (x,y): (usize, usize)) -> (usize,usize) {
        (x, y-1)
    }

    fn setup_loop_values(&self, game: &mut Game, outer_loop: usize) {
        game.set_inner_loop_counter(self.create_inner_loop_range());
        game.set_outer_loop_counter(outer_loop);
    }
}

pub struct LeftDirectionStrategy;

impl Direction for LeftDirectionStrategy{
    fn create_outer_loop_range(&self) -> Vec<usize> {
        let mut vec = Vec::with_capacity(4);
        for i in 0..4{
            vec.push(i);
        }
        vec
    }

    fn create_inner_loop_range(&self) -> usize {
        LEFT_N
    }

    fn update_loop_counter(&self, game: &mut Game) {
        let cur_count = game.get_inner_loop_counter();
        game.set_inner_loop_counter(cur_count+1);
    }

    fn get_next_value(&self, game: &mut Game) -> Option<i64> {
        let (x,y) = (*game.get_outer_loop_counter(),*game.get_inner_loop_counter());
        match game.get_mutable_board().get(x, y+1) {
            Some(a) => Some(*a),
            None => None
        }
    }

    fn evaluate_inner_loop(&self, game: &Game) -> bool {
        *game.get_inner_loop_counter() < 3
    }

    fn evaluate_row_loop(&self, game: &Game) -> bool {
        *game.get_inner_loop_counter() == 3
    }

    fn create_merge_loop_range(&self, game: &Game) -> Vec<usize> {
        let count = *game.get_inner_loop_counter();
        let mut vec = Vec::with_capacity(count);
        for i in count..4{
            vec.push(i);
        }
        vec
    }

    fn increment_value(&self, (x,y): (usize, usize)) -> (usize, usize) {
        (x, y+1)
    }

    fn setup_loop_values(&self, game: &mut Game, outer_loop: usize) {
        game.set_inner_loop_counter(self.create_inner_loop_range());
        game.set_outer_loop_counter(outer_loop);
    }
}

pub struct UpDirectionStrategy;

impl Direction for UpDirectionStrategy{
    fn create_outer_loop_range(&self) -> Vec<usize> {
        let mut vec = Vec::with_capacity(4);
        for i in 0..4{
            vec.push(i);
        }
        vec
    }

    fn create_inner_loop_range(&self) -> usize {
        UP_N
    }

    fn create_merge_loop_range(&self, game: &Game) -> Vec<usize> {
        let count = *game.get_inner_loop_counter();
        let mut vec = Vec::with_capacity(count);
        for i in count..4{
            vec.push(i);
        }
        vec
    }

    fn update_loop_counter(&self, game: &mut Game) {
        let cur_count = game.get_outer_loop_counter();
        game.set_outer_loop_counter(cur_count+1);
    }

    fn get_next_value(&self, game: &mut Game) -> Option<i64> {
        let (x,y) = (*game.get_outer_loop_counter(),*game.get_inner_loop_counter());
        match game.get_mutable_board().get(x+1, y) {
            Some(a) => Some(*a),
            None => None
        }
    }

    fn evaluate_inner_loop(&self, game: &Game) -> bool {
        *game.get_outer_loop_counter() < 3
    }

    fn evaluate_row_loop(&self, game: &Game) -> bool {
        *game.get_inner_loop_counter() == 4
    }

    fn increment_value(&self, (x,y): (usize, usize)) -> (usize,usize) {
        (x+1, y)
    }

    fn setup_loop_values(&self, game: &mut Game, outer_loop: usize) {
        game.set_inner_loop_counter(outer_loop);
        game.set_outer_loop_counter(self.create_inner_loop_range());
    }
}

pub struct DownDirectionStrategy;

impl  Direction for DownDirectionStrategy {
    fn create_outer_loop_range(&self) -> Vec<usize>{
        let mut vec = Vec::with_capacity(4);
        for i in (0..4).rev(){
            vec.push(i);
        }
        vec
    }

    fn create_inner_loop_range(&self) -> usize {
        DOWN_N
    }

    fn update_loop_counter(&self, game: &mut Game) {
        let cur_count = game.get_outer_loop_counter();
        game.set_outer_loop_counter(cur_count-1);
    }

    fn get_next_value(&self, game: &mut Game) -> Option<i64> {
        let (x,y) = (*game.get_outer_loop_counter(),*game.get_inner_loop_counter());
        match game.get_mutable_board().get(x-1, y) {
            Some(a) => Some(*a),
            None => None
        }
    }

    fn evaluate_inner_loop(&self, game: &Game) -> bool {
        *game.get_outer_loop_counter() > 0
    }

    fn evaluate_row_loop(&self, game: &Game) -> bool {
        *game.get_outer_loop_counter() == 0
    }

    fn create_merge_loop_range(&self, game: &Game) -> Vec<usize> {
        let count = *game.get_outer_loop_counter();
        let mut vec = Vec::with_capacity(count);
        for i in (0..count).rev(){
            vec.push(i);
        }
        vec
    }

    fn increment_value(&self, (x,y): (usize, usize)) -> (usize,usize) {
        (x - 1, y)
    }

    fn setup_loop_values(&self, game: &mut Game, outer_loop: usize) {
        game.set_inner_loop_counter(outer_loop);
        game.set_outer_loop_counter(self.create_inner_loop_range());
    }
}