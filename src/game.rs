

use array2d::Array2D;
use rand::Rng;

pub struct Game{
    pub board: Array2D<i64>,
    pub points: i64,
}

pub trait NormalGame{
    fn new() -> Self;
    fn initialize_board() -> Array2D<i64>;

}

impl Game{
    //returns board
    pub fn get_board(&self) ->  &Array2D<i64>{
        &self.board
    }

    pub fn get_mut_board(&mut self) -> &mut Array2D<i64>{
        &mut self.board
    }

    pub fn swipe_right(&mut self){
        let board = self.get_mut_board();
        for i in 0..4 {
            let mut n = 3;
            'inner: while n >= 0 {
                match n { // matches on column index (horizontal)
                    0 => (),
                    _ => match *board.get(i, n).unwrap() { // matches on value on (row: i, column: n)
                            0 => {
                                let element = Game::check_left(board, i, n-1);
                                if element == -1{ // -1 means loop reached the end
                                    break 'inner;
                                }
                                board.set(i, n, element).unwrap(); // set value on (row: i, column: n) to element
                                n -= 1;
                            },
                            a => {
                                let element = Game::check_left(board, i, n-1);
                                if element == -1{
                                    break 'inner; // -1 means loop reached the end
                                }
                                if element == a{
                                    board.set(i, n, element+a).unwrap();
                                    n -= 1;
                                }
                                else{
                                    board.set(i, n-1, element).unwrap();
                                    n -= 1;
                                }
                            },
                        }
                }
            }
        }
    }

    fn check_left(board: &mut Array2D<i64>, x: usize, y: usize) -> i64{
        // y is horizontal, x is vertical
        match y{
            0 => match *board.get(x, y).unwrap(){
                0 => -1, // return -1 because it has reached the end
                a => {
                    board.set(x, y, 0).unwrap();
                    a
                },
            },
            _ => match *board.get(x, y).unwrap() {
                0 => Game::check_left(board, x, y-1),
                b => {
                    board.set(x, y, 0).unwrap();
                    b
                }
            },
        }
    }

}

impl NormalGame for Game{
    //Makes a new game with an empty board and 
    fn new() -> Game{
        Game{
            board: <Game as NormalGame>::initialize_board(), 
            points: 0,
        }
    }

    // initializes board with 2 in two random spots
    fn initialize_board() -> Array2D<i64> {
        let mut rng = rand::thread_rng();
        let (mut x1, mut x2, mut y1, mut y2) = (0, 0, 0, 0);
        //While loop makes sure the two initial values don't start in the same spot
        while x1 == x2 && y1 == y2{
            (x1, x2, y1, y2) = (
                rng.gen_range(0..4),
                rng.gen_range(0..4),
                rng.gen_range(0..4),
                rng.gen_range(0..4)
            );
        } // generating random positions

        let mut board = Array2D::filled_with(0, 4, 4);
        board.set(x1, y1, 2)
            .unwrap();
        board.set(x2, y2, 2)
            .unwrap();
        board
    }
}

