//lets variables be declared but not used
#![allow(unused)]
use std::io;
use array2d::Array2D;
use remake2048::direction::Direction;
use remake2048::{game, custom_game};
use crate::game::Game;
use crate::custom_game::CustomGame;




fn main() {

    
    let mut game_obj: Game = CustomGame::new();
    game_obj.add_at_position(2, 0, 0);
    game_obj.add_at_position(2, 0, 1);
    game_obj.add_at_position(2, 0, 2);
    game_obj.add_at_position(2, 0, 3);
    let board = game_obj.get_board();
    Game::print_board(board);
    game_obj.swipe(&Direction::LEFT);
    let board = game_obj.get_board();
    Game::print_board(board);
    
}




