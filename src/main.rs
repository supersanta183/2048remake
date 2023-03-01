//lets variables be declared but not used
#![allow(unused)]
mod game;

use std::io;

use game::Game;
use crate::game::NormalGame;

fn main() {
    let mut game = game::NormalGame::new();
    print_board(game);
}

//function for testing purposes
pub fn print_board(mut game: Game){
    let array = game.get_board();
    for row_iter in array.rows_iter(){
        for element in row_iter{
            print!("{} ", element);
        }
        println!();
    }
}




