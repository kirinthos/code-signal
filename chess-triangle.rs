

enum PieceType {
    Knight,
    Bishop,
    Rook
}

struct Point(i32, i32)

struct Piece {
    piece_type: PiectType,
    p: Point,
    board_size: &Point,
}

impl Piece {
    fn new(p: Point, piece_type: PieceType, board_size: &Point) {
        Piece { p, piece_type, board_size }
    }

    fn valid_moves(&self) -> Vec<Point> {
        match &self.piece_type {
            PieceType::Knight => vec![],
            PieceType::Bishop => vec![],
            PieceType::Rook => vec![],
        }
    }
}



fn chessTriangle(n: i32, m: i32) -> i32 {
    0
}



fn main() {
    let n = 2;
    let m = 3;
    println!("({}, {}) = {}", n, m, chessTriangle(n, m));
}
