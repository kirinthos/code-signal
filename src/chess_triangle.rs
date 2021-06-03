use itertools::chain;
use itertools::iproduct;
use std::iter::repeat;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
enum PieceType {
    Knight,
    Bishop,
    Rook,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point(i32, i32);

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0 == other.0 {
            self.1.partial_cmp(&self.0)
        } else {
            self.0.partial_cmp(&self.1)
        }
    }
}

impl Point {
    fn try_new(x: i32, y: i32, board_size: &Point) -> Option<Point> {
        if x >= 0 && x < board_size.0 && y >= 0 && y < board_size.1 {
            Some(Self(x, y))
        } else {
            None
        }
    }

    fn try_from_pt(p: Point, board_size: &Point) -> Option<Point> {
        Self::try_new(p.0, p.1, board_size)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<(i32, i32)> for Point {
    fn from(p: (i32, i32)) -> Self {
        Point(p.0, p.1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    piece_type: PieceType,
    p: Point,
}

impl Piece {
    fn new(p: Point, piece_type: PieceType) -> Self {
        Self { p, piece_type }
    }

    fn next_moves(&self, board_size: &Point) -> Vec<Point> {
        match &self.piece_type {
            PieceType::Knight => self.knight_moves(board_size),
            PieceType::Bishop => self.bishop_moves(board_size),
            PieceType::Rook => self.rook_moves(board_size),
        }
    }

    fn can_attack(&self, piece: &Point, board_size: &Point) -> bool {
        self.next_moves(board_size).contains(piece)
    }

    fn knight_moves(&self, board_size: &Point) -> Vec<Point> {
        let shift = [-2i32, -1, 1, 2];
        iproduct!(&shift[..], &shift[..])
            .filter(|(x, y)| (x.abs() - y.abs()).abs() == 1)
            .filter_map(|(x, y)| Point::try_new(self.p.0 + *x, self.p.1 + *y, board_size))
            .collect()
    }

    fn bishop_moves(&self, board_size: &Point) -> Vec<Point> {
        let shift = [-1, 1];
        iproduct!(&shift[..], &shift[..])
            .flat_map(|d| {
                let directions = (1..)
                    .zip((1..))
                    .map(move |v| self.p + (v.0 * d.0, v.1 * d.1).into());
                directions
                    .map(|d| Point::try_from_pt(d, board_size))
                    .take_while(|v| v.is_some())
                    .map(|v| v.unwrap())
            })
            .collect()
    }

    fn rook_moves(&self, board_size: &Point) -> Vec<Point> {
        let shift = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        shift
            .iter()
            .flat_map(|p| {
                repeat(p)
                    .enumerate()
                    .map(|(i, (x, y))| {
                        Point::try_from_pt(
                            self.p + (x * ((i as i32) + 1), y * ((i as i32) + 1)).into(),
                            board_size,
                        )
                    })
                    .take_while(|v| v.is_some())
                    .map(|v| v.unwrap())
            })
            .collect()
    }
}

fn chessTriangle(n: i32, m: i32) -> i32 {
    // place the knight
    // for each space it can attack
    //  place the bishop, for each space it can attack
    //      place the rook, if it can attack knight - save it
    //  place the rook, for each space it can attack
    //      place the bishop, if it can attack knight - save it

    println!("{}, {}", n, m);
    let board_size = Point(n, m);
    iproduct!((0..n), (0..m))
        .flat_map(|p| {
            println!("{:?}", p);
            let knight = Piece {
                p: p.into(),
                piece_type: PieceType::Knight,
            };
            knight
                .next_moves(&board_size)
                .into_iter()
                .flat_map(move |next_p| {
                    let bishop = Piece {
                        p: next_p,
                        piece_type: PieceType::Bishop,
                    };

                    let bishop_moves =
                        bishop
                            .next_moves(&board_size)
                            .into_iter()
                            .map(move |final_p| {
                                let rook = Piece {
                                    p: final_p,
                                    piece_type: PieceType::Rook,
                                };
                                rook.can_attack(&knight.p, &board_size) as i32
                            });

                    let rook = Piece {
                        p: next_p,
                        piece_type: PieceType::Rook,
                    };

                    let rook_moves = rook
                        .next_moves(&board_size)
                        .into_iter()
                        .map(move |final_p| {
                            let bishop = Piece {
                                p: final_p,
                                piece_type: PieceType::Bishop,
                            };
                            bishop.can_attack(&knight.p, &board_size) as i32
                        });

                    chain(bishop_moves, rook_moves)
                })
        })
        .sum()
}

mod test {
    use super::*;

    use test_case::test_case;

    #[test_case(2, 3, 8)]
    #[test_case(1, 30, 0)]
    #[test_case(3, 3, 48)]
    #[test_case(2, 2, 0)]
    #[test_case(5, 2, 40)]
    fn examples(n: i32, m: i32, actual: i32) {
        let n = 2;
        let m = 3;
        let expected = 8;
        let actual = chessTriangle(n, m);
        assert_eq!(actual, expected);
    }

    #[test]
    fn knight_moves() {
        let p = Piece {
            p: Point(1, 1),
            piece_type: PieceType::Knight,
        };
        let board_size = Point(4, 4);
        let moves = p.next_moves(&board_size);
        let expected = vec![Point(0, 3), Point(2, 3), Point(3, 0), Point(3, 2)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn bishop_moves_simple() {
        let p = Piece {
            p: Point(0, 0),
            piece_type: PieceType::Bishop,
        };
        let board_size = Point(3, 3);
        let moves = p.next_moves(&board_size);
        let expected = vec![Point(1, 1), Point(2, 2)];
        assert_eq!(moves, expected);
    }

    #[test]
    fn bishop_moves_middle() {
        let p = Piece {
            p: Point(2, 2),
            piece_type: PieceType::Bishop,
        };
        let board_size = Point(5, 5);
        let moves = p.next_moves(&board_size);
        let expected = vec![
            Point(1, 1),
            Point(0, 0),
            Point(1, 3),
            Point(0, 4),
            Point(3, 1),
            Point(4, 0),
            Point(3, 3),
            Point(4, 4),
        ];
        assert_eq!(moves, expected);
    }

    #[test]
    fn rook_moves() {
        let p = Piece {
            p: Point(2, 2),
            piece_type: PieceType::Rook,
        };
        let board_size = Point(5, 5);
        let moves = p.next_moves(&board_size);
        let expected = vec![
            Point(1, 2),
            Point(0, 2),
            Point(2, 1),
            Point(2, 0),
            Point(3, 2),
            Point(4, 2),
            Point(2, 3),
            Point(2, 4),
        ];
        assert_eq!(moves, expected);
    }
}
