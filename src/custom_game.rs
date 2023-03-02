use array2d::Array2D;

use crate::game::Game;

pub trait CustomGame {
    fn new() -> Self;
    fn initialize_board() -> Array2D<i64>;
    fn add_at_position(&mut self, element: i64, x: usize, y: usize);
}

impl CustomGame for Game{
    fn new() -> Game{
        Game{
            board: Game::initialize_board(),
            points: 0,
        }
    }
    fn initialize_board() -> Array2D<i64> {
        Array2D::filled_with(0, 4, 4)
    }
    fn add_at_position(&mut self, element: i64, x: usize, y: usize){
        let board = self.get_mut_board();
        board.set(x, y, element).unwrap();
    }

}