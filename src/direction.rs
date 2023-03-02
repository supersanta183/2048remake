use array2d::Array2D;

use crate::game::Game;


const LEFT_N: usize = 0;
const RIGHT_N: usize = 3;
const UP_N: usize = 0;
const DOWN_N: usize = 3;

pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

impl Direction{
    pub fn get_n(dir: &Direction) -> usize{
        match dir{
            Direction::LEFT => LEFT_N,
            Direction::RIGHT => RIGHT_N,
            Direction::UP => UP_N,
            Direction::DOWN => DOWN_N,
        }
    }

    pub fn increment_direction(dir: &Direction, n: usize) -> usize{
        match dir{
            Direction::LEFT | Direction::UP => n + 1,
            Direction::RIGHT | Direction::DOWN => n - 1,
        }
    }

    pub fn get_value_comparison(board: &mut Array2D<i64>, x: usize, y: usize, dir: &Direction) -> Option<i64>{
        match dir{
            Direction::LEFT | Direction::UP => {
                match y {
                    3 => None,
                    _ => Game::get_value_recursive(board, x, y, dir),
                }
            },
            Direction::RIGHT | Direction::DOWN => {
                match y {
                    0 => None,
                    _ => Game::get_value_recursive(board, x, y, dir),
                }
            }
        }
    }

    pub fn swipe_direction(board: &mut Array2D<i64>, i: usize, mut n: usize, dir: &Direction) {
        match dir {
            Direction::RIGHT | Direction::DOWN => {
                while n > 0{
                    match Game::swipe_content(board, i, n, dir) {
                        Some(a) => n = a,
                        None => break,
                    }
                }
            },
            Direction::LEFT | Direction::UP => {
                while n<4{
                    match Game::swipe_content(board, i, n, dir) {
                        Some(a) => n = a,
                        None => break,
                    }
                }
            },   
        };
    }
}