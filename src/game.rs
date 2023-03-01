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
    pub fn get_board(&mut self) -> &mut Array2D<i64>{
        &mut self.board
    }

}

impl NormalGame for Game{
    //Makes a new game with an empty board and 
    fn new() -> Game{
        return Game{
            board: Game::initialize_board(), 
            points: 0,
        };
    }

    // initializes board with 2 in two random spots
    fn initialize_board() -> Array2D<i64>{
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
        }

        let mut board = Array2D::filled_with(0, 4, 4);
        board.set(x1, y1, 2)
            .unwrap();
        board.set(x2, y2, 2)
            .unwrap();
        board // return
    }
}

