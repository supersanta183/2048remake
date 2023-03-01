use crate::game::Game;

use array2d::Array2D;

pub trait CustomGame {
    fn new() -> Self;
    fn initialize_board() -> Array2D<i64>;
    fn add_at_position(self, element: i64, x: usize, y: usize);
}

impl CustomGame for Game{
    fn new() -> Game{
        return Game{
            board: Game::initialize_board(),
            points: 0,
        };
    }
    fn initialize_board() -> Array2D<i64> {
        let mut board = Array2D::filled_with(0, 4, 4);
        board
    }
    fn add_at_position(mut self: Game, element: i64, x: usize, y: usize){
        self.get_board().set(x, y, element).unwrap();
    }

}