
#[cfg(test)]
mod test_game{
    use remake2048;
    use remake2048::direction::{DirectionController, RightDirectionStrategy, LeftDirectionStrategy, UpDirectionStrategy, DownDirectionStrategy};
    use remake2048::game::{NormalGame, Game, CustomGame};

    struct Setup{
        normal_game: Game,
        custom_game: Game,
    }

    impl Setup{
        fn new() -> Self{
            Self { normal_game: NormalGame::new(), custom_game: CustomGame::new() }
        }
    }

#[test]
fn board_should_be_i64(){
    let mut setup = Setup::new();

    assert_eq!(setup.normal_game.get_board().num_elements(), 16)
}

#[test]
fn when_inserted_on_0_3_it_is_2(){
    let mut setup = Setup::new();
    setup.custom_game.add_at_position(2, 0, 3);
    let board = setup.custom_game.get_board();
    let x = board[(0,3)];
    assert_eq!(x,2);
}

#[test]
// 2 0 0 0 is inserted, shoudl be 0 0 0 2 after test
fn swipe_moves_2_to_the_right_most_column(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.swipe(&DirectionController::new(RightDirectionStrategy));
    let board = setup.custom_game.get_board();
    let x = board[(0,3)];

    assert_eq!(x, 2);
}


//TEST SWIPE RIGHT
#[test]
// 2 2 0 0 is inserted, should be 0 0 0 4 after swipe right
fn when_swiping_add_2_equal_numbers_from_position_1_and_2() {
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(2, 0, 1);
    setup.custom_game.swipe(&DirectionController::new(RightDirectionStrategy));
    let board = setup.custom_game.get_board();
    let x = board[(0,3)];

    assert_eq!(x, 4);
}

#[test]
// 2 0 4 0 is inserted, should be 0 0 2 4 after swipe
fn insert_2_0_4_0_should_not_merge(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(4, 0, 2);
    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.swipe(&DirectionController::new(RightDirectionStrategy));
    let board = setup.custom_game.get_board();

    assert_eq!(board[(0,3)], 4);
    assert_eq!(board[(0,2)], 2);
}

#[test]
// 2 0 2 0 is inserted, should be 0 0 0 4 after swiping right
fn insert_2_0_2_0_should_merge(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 2);
    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.swipe(&DirectionController::new(RightDirectionStrategy));
    let board = setup.custom_game.get_board();

    assert_eq!(board[(0,3)], 4);
}

#[test]
// 0 2 2 0 is inserted, should be 0 0 0 4 after swipe right
fn when_swiping_adds_2_equal_numbers(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 2);
    setup.custom_game.add_at_position(2, 0, 1);
    setup.custom_game.swipe(&DirectionController::new(RightDirectionStrategy));
    let board = setup.custom_game.get_board();
    let x = board[(0,3)];

    assert_eq!(x, 4);
}

#[test]
// 2 4 2 0 is inserted, shoudl be 0 2 4 2 after swipe right
fn swipes_correctly_when_2_4_2(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(4, 0, 1);
    setup.custom_game.add_at_position(2, 0, 2);
    setup.custom_game.add_at_position(0, 0, 3);
    setup.custom_game.swipe(&DirectionController::new(RightDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(0,3)], 2);
    assert_eq!(board[(0,2)],4);
    assert_eq!(board[(0,1)], 2);
}

#[test]
// 2 2 2 2 is inserted, should be 0 0 4 4 after swipe right
fn swipes_correctly_when_2_2_2_2(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(2, 0, 1);
    setup.custom_game.add_at_position(2, 0, 2);
    setup.custom_game.add_at_position(2, 0, 3);
    setup.custom_game.swipe(&DirectionController::new(RightDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(0,3)], 4);
    assert_eq!(board[(0,2)],4);
}

#[test]
fn swipes_correctly_when_2_4_2_4(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(4, 0, 1);
    setup.custom_game.add_at_position(2, 0, 2);
    setup.custom_game.add_at_position(4, 0, 3);
    setup.custom_game.swipe(&DirectionController::new(RightDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(0,3)], 4);
    assert_eq!(board[(0,2)], 2);
    assert_eq!(board[(0,1)], 4);
    assert_eq!(board[(0,0)], 2);
}

//TEST SWIPE LEFT
#[test]
// 2 4 2 4 is inserted, should be same after test
fn swipes_correctly_when_2_4_2_4_left(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(4, 0, 1);
    setup.custom_game.add_at_position(2, 0, 2);
    setup.custom_game.add_at_position(4, 0, 3);
    setup.custom_game.swipe(&DirectionController::new(LeftDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(0,3)], 4);
    assert_eq!(board[(0,2)], 2);
    assert_eq!(board[(0,1)], 4);
    assert_eq!(board[(0,0)], 2);
}

#[test]
// 2 2 2 2 is inserted, should be 4 4 0 0 after swipe right
fn swipes_correctly_when_2_2_2_2_left(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(2, 0, 1);
    setup.custom_game.add_at_position(2, 0, 2);
    setup.custom_game.add_at_position(2, 0, 3);
    setup.custom_game.swipe(&DirectionController::new(LeftDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(0,0)], 4);
    assert_eq!(board[(0,1)], 4);
}

#[test]
// 0 2 2 0 is inserted, should be 4 0 0 0 after swipe right
fn when_swiping_adds_2_equal_numbers_left(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 2);
    setup.custom_game.add_at_position(2, 0, 1);
    setup.custom_game.swipe(&DirectionController::new(LeftDirectionStrategy));
    let board = setup.custom_game.get_board();

    assert_eq!(board[(0,0)], 4);
}

#[test]
// 0 2 4 2 is inserted, shoudl be 2 4 2 0 after swipe right
fn swipes_correctly_when_2_4_2_left(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(0, 0, 0);
    setup.custom_game.add_at_position(2, 0, 1);
    setup.custom_game.add_at_position(4, 0, 2);
    setup.custom_game.add_at_position(2, 0, 3);
    setup.custom_game.swipe(&DirectionController::new(LeftDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(0,0)], 2);
    assert_eq!(board[(0,1)], 4);
    assert_eq!(board[(0,2)], 2);
}

//TESTS FOR SWIPING UP
#[test]
// 2 2 0 0 is inserted, should be 0 0 0 4 after swipe right
fn when_swiping_add_2_equal_numbers_from_position_1_and_2_up() {
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 1, 0);
    setup.custom_game.add_at_position(2, 2, 0);
    setup.custom_game.swipe(&DirectionController::new(UpDirectionStrategy));
    let board = setup.custom_game.get_board();
    let x = board[(0,0)];

    assert_eq!(x, 4);
}

#[test]
// 2 4 2 0 is inserted, shoudl be 0 2 4 2 after swipe right
fn swipes_correctly_when_2_4_2_up(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 3, 0);
    setup.custom_game.add_at_position(4, 2, 0);
    setup.custom_game.add_at_position(2, 1, 0);
    setup.custom_game.add_at_position(0, 0, 0);
    setup.custom_game.swipe(&DirectionController::new(UpDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(0,0)], 2);
    assert_eq!(board[(1,0)],4);
    assert_eq!(board[(2,0)], 2);
}

#[test]
// 2 2 2 2 is inserted, should be 0 0 4 4 after swipe right
fn swipes_correctly_when_2_2_2_2_up(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(2, 1, 0);
    setup.custom_game.add_at_position(2, 2, 0);
    setup.custom_game.add_at_position(2, 3, 0);
    setup.custom_game.swipe(&DirectionController::new(UpDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(0,0)], 4);
    assert_eq!(board[(1,0)], 4);
}

#[test]
fn swipes_correctly_when_2_4_2_4_up(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(4, 1, 0);
    setup.custom_game.add_at_position(2, 2, 0);
    setup.custom_game.add_at_position(4, 3, 0);
    setup.custom_game.swipe(&DirectionController::new(UpDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(3,0)], 4);
    assert_eq!(board[(2,0)], 2);
    assert_eq!(board[(1,0)], 4);
    assert_eq!(board[(0,0)], 2);
}

#[test]
// 2 0 2 0 is inserted vertically, should be 0 0 0 4 after swiping right
fn insert_2_0_2_0_should_merge_up(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(2, 2, 0);
    setup.custom_game.swipe(&DirectionController::new(UpDirectionStrategy));
    let board = setup.custom_game.get_board();

    assert_eq!(board[(0,0)], 4);
}

//TESTS FOR SWIPING DOWN
#[test]
// 2 2 0 0 is inserted, should be 0 0 0 4 after swipe right
fn when_swiping_add_2_equal_numbers_from_position_1_and_2_down() {
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 1, 0);
    setup.custom_game.add_at_position(2, 2, 0);
    setup.custom_game.swipe(&DirectionController::new(DownDirectionStrategy));
    let board = setup.custom_game.get_board();
    let x = board[(3,0)];

    assert_eq!(x, 4);
}

#[test]
// 2 4 2 0 is inserted, shoudl be 0 2 4 2 after swipe right
fn swipes_correctly_when_2_4_2_down(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 3, 0);
    setup.custom_game.add_at_position(4, 2, 0);
    setup.custom_game.add_at_position(2, 1, 0);
    setup.custom_game.add_at_position(0, 0, 0);
    setup.custom_game.swipe(&DirectionController::new(DownDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(1,0)], 2);
    assert_eq!(board[(2,0)],4);
    assert_eq!(board[(3,0)], 2);
}

#[test]
// 2 2 2 2 is inserted, should be 0 0 4 4 after swipe right
fn swipes_correctly_when_2_2_2_2_down(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(2, 1, 0);
    setup.custom_game.add_at_position(2, 2, 0);
    setup.custom_game.add_at_position(2, 3, 0);
    setup.custom_game.swipe(&DirectionController::new(DownDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(2,0)], 4);
    assert_eq!(board[(3,0)], 4);
}

#[test]
fn swipes_correctly_when_2_4_2_4_down(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(4, 1, 0);
    setup.custom_game.add_at_position(2, 2, 0);
    setup.custom_game.add_at_position(4, 3, 0);
    setup.custom_game.swipe(&DirectionController::new(DownDirectionStrategy));
    let board = setup.custom_game.get_board();
    assert_eq!(board[(3,0)], 4);
    assert_eq!(board[(2,0)], 2);
    assert_eq!(board[(1,0)], 4);
    assert_eq!(board[(0,0)], 2);
}

}


