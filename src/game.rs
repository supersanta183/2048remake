use array2d::Array2D;
use rand::Rng;


pub struct Game{
    board: Array2D<i64>,
    points: i64,
}

// generates 2 random starting positions for the board 
fn generate_starting_position() -> (i32, i32, i32,i32){
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
    (x1, x2, y1, y2)
}


impl Game{
    //Makes a new game with an empty board and 
    pub fn new() -> Game{
        return Game{
            board: Game::initialize_board(), 
            points: 0,
        };
    }

    //returns board
    pub fn get_board(&mut self) -> &mut Array2D<i64>{
        &mut self.board
    }

    // initializes board with 2 in two random spots
    fn initialize_board() -> Array2D<i64>{
        let (x1, x2, y1, y2) = generate_starting_position();
        let mut board = Array2D::filled_with(0, 4, 4);
        board.set(x1 as usize, y1 as usize, 2)
            .unwrap_or_else(|error|{ // Error handling
                println!("An error has occured while initializing board, attempting to rerun : {:?}", error);
                Game::new();
            });
        board.set(x2 as usize, y2 as usize, 2)
            .unwrap_or_else(|error|{ // Error handling
                println!("An error has occured while initializing board, attempting to rerun : {:?}", error);
                Game::new();
            });

        board // return
    }
}

