pub const BOARD_SIZE: usize = 19;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Piece {
    Black,
    White,
}

struct Board {
    contents: Vec<Vec<Option<Piece>>>,
}

impl Board {
    fn new() -> Board {
        let v: Vec<Vec<Option<Piece>>> = vec![vec![None; BOARD_SIZE]; BOARD_SIZE];
        Board { contents: v }
    }
}
struct MainState {
    board: Board,
    current_turn: Piece,
}