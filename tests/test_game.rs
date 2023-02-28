use remake2048;

#[test]
fn board_should_be_i64(){
    let mut game = remake2048::game::Game::new();

    assert_eq!(game.get_board().num_elements(), 16)
}

#[test]
fn position_0_0_is_0(){
    let mut game = remake2048::game::Game::new();

    assert_eq!(game.get_board().get(0, 0), Some(&0));
}