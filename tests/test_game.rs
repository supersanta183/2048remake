
#[cfg(test)]
mod test_game{
    use remake2048;
    use remake2048::direction::Direction;
    use remake2048::game::{NormalGame, Game};
    use remake2048::custom_game::CustomGame;

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
fn swipe_moves_2_to_the_right_most_column(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.swipe(&Direction::RIGHT);
    let board = setup.custom_game.get_board();
    let x = board[(0,3)];

    assert_eq!(x, 2);
}

#[test]
// 2 2 0 0 is inserted, should be 0 0 0 4 after swipe right
fn when_swiping_add_2_equal_numbers_from_position_1_and_2() {
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 0);
    setup.custom_game.add_at_position(2, 0, 1);
    setup.custom_game.swipe(&Direction::RIGHT);
    let board = setup.custom_game.get_board();
    let x = board[(0,3)];

    assert_eq!(x, 4);
}

#[test]
// 0 2 2 0 is inserted, should be 0 0 0 4 after swipe right
fn when_swiping_adds_2_equal_numbers(){
    let mut setup = Setup::new();

    setup.custom_game.add_at_position(2, 0, 2);
    setup.custom_game.add_at_position(2, 0, 1);
    setup.custom_game.swipe(&Direction::RIGHT);
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
    setup.custom_game.swipe(&Direction::RIGHT);
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
    setup.custom_game.swipe(&Direction::RIGHT);
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
    setup.custom_game.swipe(&Direction::RIGHT);
    let board = setup.custom_game.get_board();
    assert_eq!(board[(0,3)], 4);
    assert_eq!(board[(0,2)], 2);
    assert_eq!(board[(0,1)], 4);
    assert_eq!(board[(0,0)], 2);
}
}


