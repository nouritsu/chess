use crate::board::{ChessBoard, BOARD_SIZE};
use pleco::{Piece, SQ};

pub fn get_valid_moves(board: &ChessBoard, sq: SQ, piece: Piece) -> Vec<SQ> {
    todo!()
}

pub fn get_moves(board: &ChessBoard, sq: SQ, piece: Piece) -> Vec<SQ> {
    match piece {
        Piece::WhitePawn | Piece::BlackPawn => {
            let (_, y) = Direction::sq_to_xy(sq);
            let direction = match piece {
                Piece::WhitePawn => Direction::PawnUp(y == 1),
                Piece::BlackPawn => Direction::PawnDown(y == 6),
                _ => unreachable!(),
            };

            vec![direction]
        }

        Piece::WhiteKnight | Piece::BlackKnight => vec![Direction::Knight],

        Piece::WhiteBishop | Piece::BlackBishop => vec![
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
        .into_iter()
        .map(|dir| dir(false))
        .collect(),

        Piece::WhiteRook | Piece::BlackRook => vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .map(|dir| dir(false))
        .collect(),

        Piece::WhiteQueen | Piece::BlackQueen => vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
        .into_iter()
        .map(|dir| dir(false))
        .collect(),

        Piece::WhiteKing | Piece::BlackKing => vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
        .iter()
        .map(|dir| dir(true))
        .collect(),

        Piece::None => unreachable!(),
    }
    .apply(board, sq)
}

enum Direction {
    Up(/* single */ bool),
    Down(/* single */ bool),
    Left(/* single */ bool),
    Right(/* single */ bool),
    UpLeft(/* single */ bool),
    UpRight(/* single */ bool),
    DownLeft(/* single */ bool),
    DownRight(/* single */ bool),
    PawnUp(/* first_move */ bool),
    PawnDown(/* first_move */ bool),
    Knight,
}

trait ApplyDirections {
    fn apply(self, board: &ChessBoard, sq: SQ) -> Vec<SQ>;
}

impl ApplyDirections for Vec<Direction> {
    fn apply(self, board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        self.into_iter()
            .flat_map(|dir| dir.apply(board, sq))
            .collect()
    }
}

impl Direction {
    pub fn apply(self, board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        match self {
            Direction::Up(true) => Self::up(board, sq).into_iter().take(1).collect(),
            Direction::Down(true) => Self::down(board, sq).into_iter().take(1).collect(),
            Direction::Left(true) => Self::left(board, sq).into_iter().take(1).collect(),
            Direction::Right(true) => Self::right(board, sq).into_iter().take(1).collect(),
            Direction::UpLeft(true) => Self::up_left(board, sq).into_iter().take(1).collect(),
            Direction::UpRight(true) => Self::up_right(board, sq).into_iter().take(1).collect(),
            Direction::DownLeft(true) => Self::down_left(board, sq).into_iter().take(1).collect(),
            Direction::DownRight(true) => Self::down_right(board, sq).into_iter().take(1).collect(),

            Direction::Up(false) => Self::up(board, sq),
            Direction::Down(false) => Self::down(board, sq),
            Direction::Left(false) => Self::left(board, sq),
            Direction::Right(false) => Self::right(board, sq),
            Direction::UpLeft(false) => Self::up_left(board, sq),
            Direction::UpRight(false) => Self::up_right(board, sq),
            Direction::DownLeft(false) => Self::down_left(board, sq),
            Direction::DownRight(false) => Self::down_right(board, sq),

            Direction::PawnUp(true) => Self::pawn_up(board, sq),
            Direction::PawnDown(true) => Self::pawn_down(board, sq),

            Direction::PawnUp(false) => Self::pawn_up(board, sq).into_iter().take(1).collect(),
            Direction::PawnDown(false) => Self::pawn_down(board, sq).into_iter().take(1).collect(),

            Direction::Knight => Self::knight(board, sq),
        }
    }

    fn up(board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        let (x, mut y) = Self::sq_to_xy(sq);
        let mut moves = vec![];

        while y < BOARD_SIZE {
            y = y.wrapping_add(1);

            let sq = Self::xy_to_sq(x, y);
            moves.push(sq);

            if board.get_piece_locations().at_square(sq) {
                break;
            }
        }

        moves
    }

    fn down(board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        let (x, mut y) = Self::sq_to_xy(sq);
        let mut moves = vec![];

        while y > 0 {
            y = y.wrapping_sub(1);

            let sq = Self::xy_to_sq(x, y);
            moves.push(sq);

            if board.get_piece_locations().at_square(sq) {
                break;
            }
        }

        moves
    }

    fn left(board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        let (mut x, y) = Self::sq_to_xy(sq);
        let mut moves = vec![];

        while x > 0 {
            x = x.wrapping_sub(1);

            let sq = Self::xy_to_sq(x, y);
            moves.push(sq);

            if board.get_piece_locations().at_square(sq) {
                break;
            }
        }

        moves
    }

    fn right(board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        let (mut x, y) = Self::sq_to_xy(sq);
        let mut moves = vec![];

        while x < BOARD_SIZE {
            x = x.wrapping_add(1);

            let sq = Self::xy_to_sq(x, y);
            moves.push(sq);

            if board.get_piece_locations().at_square(sq) {
                break;
            }
        }

        moves
    }

    fn up_left(board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        let (mut x, mut y) = Self::sq_to_xy(sq);
        let mut moves = vec![];

        while x > 0 && y < BOARD_SIZE {
            x = x.wrapping_sub(1);
            y = y.wrapping_add(1);

            let sq = Self::xy_to_sq(x, y);
            moves.push(sq);

            if board.get_piece_locations().at_square(sq) {
                break;
            }
        }

        moves
    }

    fn up_right(board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        let (mut x, mut y) = Self::sq_to_xy(sq);
        let mut moves = vec![];

        while x < BOARD_SIZE && y < BOARD_SIZE {
            x = x.wrapping_add(1);
            y = y.wrapping_add(1);

            let sq = Self::xy_to_sq(x, y);
            moves.push(sq);

            if board.get_piece_locations().at_square(sq) {
                break;
            }
        }

        moves
    }

    fn down_left(board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        let (mut x, mut y) = Self::sq_to_xy(sq);
        let mut moves = vec![];

        while x > 0 && y > 0 {
            x = x.wrapping_sub(1);
            y = y.wrapping_sub(1);

            let sq = Self::xy_to_sq(x, y);
            moves.push(sq);

            if board.get_piece_locations().at_square(sq) {
                break;
            }
        }

        moves
    }

    fn down_right(board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        let (mut x, mut y) = Self::sq_to_xy(sq);
        let mut moves = vec![];

        while x < BOARD_SIZE && y > 0 {
            x = x.wrapping_add(1);
            y = y.wrapping_sub(1);

            let sq = Self::xy_to_sq(x, y);
            moves.push(sq);

            if board.get_piece_locations().at_square(sq) {
                break;
            }
        }

        moves
    }

    fn knight(_: &ChessBoard, sq: SQ) -> Vec<SQ> {
        let (x, y) = Self::sq_to_xy(sq);
        let mut moves = vec![];

        let knight_moves = [
            (x.wrapping_add(1), y.wrapping_add(2)),
            (x.wrapping_add(2), y.wrapping_add(1)),
            (x.wrapping_add(2), y.wrapping_sub(1)),
            (x.wrapping_add(1), y.wrapping_sub(2)),
            (x.wrapping_sub(1), y.wrapping_sub(2)),
            (x.wrapping_sub(2), y.wrapping_sub(1)),
            (x.wrapping_sub(2), y.wrapping_add(1)),
            (x.wrapping_sub(1), y.wrapping_add(2)),
        ];

        for (x, y) in knight_moves {
            if x < BOARD_SIZE && y < BOARD_SIZE {
                moves.push(sq);
            }
        }

        moves
    }

    fn pawn_up(board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        match *Self::up(board, sq).as_slice() {
            [sq] => vec![sq],
            [sq1, sq2, ..] => vec![sq1, sq2],
            _ => vec![],
        }
    }

    fn pawn_down(board: &ChessBoard, sq: SQ) -> Vec<SQ> {
        match *Self::down(board, sq).as_slice() {
            [sq] => vec![sq],
            [sq1, sq2, ..] => vec![sq1, sq2],
            _ => vec![],
        }
    }

    fn sq_to_xy(sq: SQ) -> (u8, u8) {
        (sq.file() as u8, sq.rank() as u8)
    }

    fn xy_to_sq(x: u8, y: u8) -> SQ {
        SQ(x + y * BOARD_SIZE)
    }
}
