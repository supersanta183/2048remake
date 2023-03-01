
#[cfg(test)]
mod test_game{
    use remake2048;
    use remake2048::game::{NormalGame, Game};
    use crate::custom_game::CustomGame;

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
    let x = board.get(0, 3).unwrap();
    assert_eq!(*x,2);
}


}
