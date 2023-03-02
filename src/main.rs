//lets variables be declared but not used
#![allow(unused)]
use std::io;
use array2d::Array2D;
use remake2048::direction::{Direction, DirectionValues};
use remake2048::{game, custom_game};
use crate::game::Game;
use crate::custom_game::CustomGame;




fn main() {

    
    let mut game_obj: Game = CustomGame::new();
    game_obj.add_at_position(2, 1, 0);
    game_obj.add_at_position(2, 1, 1);
    game_obj.add_at_position(4, 1, 2);
    game_obj.add_at_position(8, 1, 3);
    let board = game_obj.get_board();
    Game::print_board(board);
    game_obj.swipe(DirectionValues::RIGHT);
    let board = game_obj.get_board();
    Game::print_board(board);
    
}




