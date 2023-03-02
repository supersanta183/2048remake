//lets variables be declared but not used
#![allow(unused)]
use std::io;
use array2d::Array2D;
use remake2048::{game, custom_game};
use crate::game::Game;
use crate::custom_game::CustomGame;




fn main() {

    
    let mut game_obj: Game = CustomGame::new();
    game_obj.add_at_position(4, 3, 0);
    game_obj.add_at_position(4, 3, 3);
    game_obj.add_at_position(4, 3, 2);
    //game_obj.add_at_position(4, 0, 3);
    //game_obj.add_at_position(4, 1, 1);
    game_obj.swipe_right();
    let board = game_obj.get_board();
    print_board(board);
    
}

//function for testing purposes
pub fn print_board(board : &Array2D<i64>){
    for row_iter in board.rows_iter(){
        for element in row_iter{
            print!("{} ", element);
        }
        println!();
    }
}




