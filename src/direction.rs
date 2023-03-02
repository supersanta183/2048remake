use crate::game::Game;


const LEFT_N: usize = 0;
const RIGHT_N: usize = 3;
const UP_N: usize = 3;
const DOWN_N: usize = 0;

pub enum DirectionValues {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

pub struct Direction{
    direction: DirectionValues,
}

impl Direction{
    pub fn new(dir: DirectionValues) -> Direction{
        Direction { direction: dir }
    }

    pub fn get_n(&self) -> usize{
        match self.direction{
            DirectionValues::LEFT => LEFT_N,
            DirectionValues::RIGHT => RIGHT_N,
            DirectionValues::UP => UP_N,
            DirectionValues::DOWN => DOWN_N,
        }
    }

    pub fn increment_direction(&self, i: usize, n: usize) -> (usize, usize){
        match self.direction{
            DirectionValues::LEFT => (i,n + 1),
            DirectionValues::RIGHT => (i,n - 1),
            DirectionValues::UP => (i+1,n),
            DirectionValues::DOWN => (i,n),
        }
    }

    pub fn get_value_comparison(&self, game: &mut Game, x: usize, y: usize) -> Option<i64>{
        match self.direction{
            DirectionValues::LEFT | DirectionValues::DOWN => {
                match y {
                    3 => None,
                    _ => game.get_value_recursive(x, y, self),
                }
            },
            DirectionValues::RIGHT => {
                match y {
                    0 => None,
                    _ => game.get_value_recursive(x, y, self),
                }
            }
            DirectionValues::UP => {
                match y{
                    3 => None,
                    _ => {
                        game.get_value_recursive(x, y, self)},
                }
            }
        }
    }

    pub fn swipe_direction(&self, game: &mut Game, mut i: usize, mut n: usize) {
        match self.direction {
            DirectionValues::RIGHT => {
                println!("n : {}", n);
                while n > 0{
                    match game.swipe_content(i, n, self) {
                        Some((a,b)) => (i,n) = (a,b),
                        None => break,
                    }
                }
            },
            DirectionValues::LEFT | DirectionValues::DOWN => {
                while n<3{
                    match game.swipe_content(i, n, self) {
                        Some((a,b)) => (i,n) = (a,b),
                        None => break,
                    }
                }
            },
            DirectionValues::UP => while i < 3{
                println!("hej");

                match game.swipe_content(i, n, self) {
                    Some((a,b)) => (i,n) = (a,b),
                    None => break,
                }
            }
        };
    }
}