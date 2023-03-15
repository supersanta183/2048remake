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

        user_input = user_input.trim().to_string();

        if user_input == "w"{
            game_obj.swipe(&DirectionController::new(UpDirectionStrategy));
        }
        if user_input == "a"{
            println!("Swiping left");
            game_obj.swipe(&DirectionController::new(LeftDirectionStrategy))
        }
        if user_input == "s"{
            game_obj.swipe(&DirectionController::new(DownDirectionStrategy))
        }
        if user_input == "d"{
            game_obj.swipe(&DirectionController::new(RightDirectionStrategy))
        }
        if user_input == "p"{
            break;
        }
    }
    
}




