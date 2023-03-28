//lets variables be declared but not used
#![allow(unused)]
use std::io;
use array2d::Array2D;
use remake2048::direction::{DirectionValues, DirectionController, RightDirectionStrategy, LeftDirectionStrategy, UpDirectionStrategy, DownDirectionStrategy};
use remake2048::game::{Game, CustomGame, NormalGame};

fn main() {
    let mut game_obj: Game = NormalGame::new();
    
    while !game_obj.get_loser_status() {
        let mut user_input = String::new();
        let board = &game_obj.board;
        println!("points : {}", game_obj.get_points());
        Game::print_board(board);
        io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

        match user_input.trim() {
            "w" => game_obj.swipe(&DirectionController::new(UpDirectionStrategy)),
            "a" => game_obj.swipe(&DirectionController::new(LeftDirectionStrategy)),
            "s" => game_obj.swipe(&DirectionController::new(DownDirectionStrategy)),
            "d" => game_obj.swipe(&DirectionController::new(RightDirectionStrategy)),
            "p" => break,
            _ => (),
        }
    }
    
}




