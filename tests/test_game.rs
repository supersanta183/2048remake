use remake2048::{self, game::{Game, NormalGame}};

fn setup() -> Game{
    remake2048::game::NormalGame::new()
}

#[test]
fn board_should_be_i64(){
    let mut game: Game = remake2048::game::NormalGame::new();

    assert_eq!(game.get_board().num_elements(), 16)
}

#[test]
fn When_inserted_on_0_3_it_is_2(){

    
}
