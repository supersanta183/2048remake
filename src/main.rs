//lets variables be declared but not used
#![allow(unused)]
use std::io;
use array2d::Array2D;
use remake2048::direction::{DirectionValues, DirectionController, RightDirectionStrategy, LeftDirectionStrategy, UpDirectionStrategy, DownDirectionStrategy};
use remake2048::game::{Game, CustomGame};




fn main() {

    
    let mut game_obj: Game = CustomGame::new();
    game_obj.add_at_position(2, 0, 0);
    game_obj.add_at_position(2, 1, 0);
    game_obj.add_at_position(2, 2, 0);
    game_obj.add_at_position(2, 3, 0);
    let board = &game_obj.board;
    Game::print_board(board);
    game_obj.swipe(&DirectionController::new(DownDirectionStrategy));
    //game_obj.swipe(&DirectionController::new(RightDirectionStrategy));
    let board = &game_obj.board;
    Game::print_board(board);
}




