use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

pub fn check_allowed_moves(board: Board, piece: (i8, i8)) -> Vec<(i8, i8)> {

    

    vec![(5, 5)] // Placeholder
}

pub fn make_move(board: Board, from: (i8, i8), to: (i8, i8)) -> (Board, State) {

    (board, State::Playing) // Placeholder
}

// checks possible moves vertically and horizontally
fn check_vertical_and_horizontal(board: [[Piece; 8]; 8], position: (i8, i8), color: Color, limit: bool) -> Vec<(i8, i8)> {
    let mut allowed_moves: Vec<(i8, i8)> = Vec::new();

    let mut right: bool = false;
    let mut left: bool = false;
    let mut up: bool = false;
    let mut down: bool = false;

    let steps: i8 = if limit {1} else {8};

    let (x, y) = position;
    
    for n in 1..steps {
        if right && left && up && down {
            break;
        }

        if !right {
            let new_x: i8 = x + n;
            if new_x >= 0 && new_x <= 7{ 
                let new_position: Piece = board[new_x as usize][y as usize];
                if new_position.piece_type == PieceType::Empty {
                    allowed_moves.push((x, y));
                }
                else if new_position.color != color {
                    allowed_moves.push((x, y));
                    right = true;
                }
                else if new_position.color == color {
                    right = true;
                }
            }
            else {
                right = true;
            }
        }
        if !left {
            let new_x: i8 = x - n;
            if new_x >= 0 && new_x <= 7 { 
                let new_position: Piece = board[new_x as usize][y as usize];
                if new_position.piece_type == PieceType::Empty {
                    allowed_moves.push((x, y));
                }
                else if new_position.color != color {
                    allowed_moves.push((x, y));
                    left = true;
                }
                else if new_position.color == color {
                    left = true;
                }
            }
            else {
                left = true;
            }
        }
        if !up {
            let new_y: i8 = y + n;
            if new_y >= 0 && new_y <= 7 { 
                let new_position: Piece = board[x as usize][new_y as usize];
                if new_position.piece_type == PieceType::Empty {
                    allowed_moves.push((x, y));
                }
                else if new_position.color != color {
                    allowed_moves.push((x, y));
                    up = true;
                }
                else if new_position.color == color {
                    up = true;
                }
            }
            else {
                up = true;
            }
        }
        if !down {
            let new_y: i8 = y - n;
            if new_y >= 0 && new_y <= 7 { 
                let new_position: Piece = board[x as usize][new_y as usize];
                if new_position.piece_type == PieceType::Empty {
                    allowed_moves.push((x, y));
                }
                else if new_position.color != color {
                    allowed_moves.push((x, y));
                    down = true;
                }
                else if new_position.color == color {
                    down = true;
                }
            }
            else {
                down = true;
            }
        }
    }

    allowed_moves
}

// checks possible moves diagonally
fn check_diagonal(board: [[Piece; 8]; 8], position: (i8, i8), color: Color, limit: bool) -> Vec<(i8, i8)> {
    let mut allowed_moves: Vec<(i8, i8)> = Vec::new();

    let mut stop_down_left: bool = false;
    let mut stop_down_right: bool = false;
    let mut stop_up_left: bool = false;
    let mut stop_up_right: bool = false;

    let steps: i8 = if limit {1} else {8};

    let (x, y) = position;
    
    for n in 1..steps {
        if stop_down_left && stop_down_right && stop_up_left && stop_up_right {
            break;
        }

        if !stop_down_left {
            let new_x: i8 = x - n;
            let new_y: i8 = y - n;
            if new_x >= 0 && new_x <= 7 && new_y >= 0 && new_y <= 7 { 
                let new_position: Piece = board[new_x as usize][new_y as usize];
                if new_position.piece_type == PieceType::Empty {
                    allowed_moves.push((x, y));
                }
                else if new_position.color != color {
                    allowed_moves.push((x, y));
                    stop_down_left = true;
                }
                else if new_position.color == color {
                    stop_down_left = true;
                }
            }
            else {
                stop_down_left = true;
            }
        }
        if !stop_down_right {
            let new_x: i8 = x - n;
            let new_y: i8 = y - n;
            if new_x >= 0 && new_x <= 7 && new_y >= 0 && new_y <= 7 { 
                let new_position: Piece = board[new_x as usize][new_y as usize];
                if new_position.piece_type == PieceType::Empty {
                    allowed_moves.push((x, y));
                }
                else if new_position.color != color {
                    allowed_moves.push((x, y));
                    stop_down_left = true;
                }
                else if new_position.color == color {
                    stop_down_left = true;
                }
            }
            else {
                stop_down_right = true;
            }
        }
        if !stop_up_right {
            let new_x: i8 = x - n;
            let new_y: i8 = y - n;
            if new_x >= 0 && new_x <= 7 && new_y >= 0 && new_y <= 7 { 
                let new_position: Piece = board[new_x as usize][new_y as usize];
                if new_position.piece_type == PieceType::Empty {
                    allowed_moves.push((x, y));
                }
                else if new_position.color != color {
                    allowed_moves.push((x, y));
                    stop_down_left = true;
                }
                else if new_position.color == color {
                    stop_down_left = true;
                }
            }
            else {
                stop_up_right = true;
            }
        }
        if !stop_up_left {
            let new_x: i8 = x - n;
            let new_y: i8 = y - n;
            if new_x >= 0 && new_x <= 7 && new_y >= 0 && new_y <= 7 { 
                let new_position: Piece = board[new_x as usize][new_y as usize];
                if new_position.piece_type == PieceType::Empty {
                    allowed_moves.push((x, y));
                }
                else if new_position.color != color {
                    allowed_moves.push((x, y));
                    stop_down_left = true;
                }
                else if new_position.color == color {
                    stop_down_left = true;
                }
            }
            else {
                stop_up_left = true;
            }
        }
    }

    allowed_moves
}

// creates a board with start positions
pub fn create_board() -> Board {
    // crates an 8x8 array of empt pieces
    let empty_piece: Piece = Piece {
        piece_type: PieceType::Empty,
        color: Color::White
    };
    let mut board_whith_piecies: [[Piece; 8]; 8] = [[empty_piece; 8]; 8];
    let mut white_protected: [[bool; 8]; 8] = [[false; 8]; 8];
    let mut black_protected: [[bool; 8]; 8] = [[false; 8]; 8];

    // loop through x positions and piecies
    for (position, piece) in (0..5).zip(PieceType::iter()) {
        board_whith_piecies[position][0] = Piece { piece_type: piece, color: Color::White };
        board_whith_piecies[position][1] = Piece { piece_type: PieceType::Pawn, color: Color::White };

        board_whith_piecies[position][7] = Piece { piece_type: piece, color: Color::Black };
        board_whith_piecies[position][6] = Piece { piece_type: PieceType::Pawn, color: Color::Black };
    }
    for (position, piece) in (8..5).zip(PieceType::iter()) {
        board_whith_piecies[position][0] = Piece { piece_type: piece, color: Color::White };
        board_whith_piecies[position][1] = Piece { piece_type: PieceType::Pawn, color: Color::White };

        board_whith_piecies[position][7] = Piece { piece_type: piece, color: Color::Black };
        board_whith_piecies[position][6] = Piece { piece_type: PieceType::Pawn, color: Color::Black };
    }

    for x in 0..8 {
        white_protected[x][1] = true;
        black_protected[x][6] = true;
    }

    Board {
        board: board_whith_piecies,
        white_protected: white_protected,
        black_protected: black_protected,
        state: State::Playing
    }
}

pub struct Board {
    pub board: [[Piece; 8]; 8],
    pub white_protected: [[bool; 8]; 8],
    pub black_protected: [[bool; 8]; 8],
    pub state: State
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone, Debug, EnumIter, PartialEq)]
pub enum PieceType {
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
    Pawn,
    Empty
}

pub enum State {
    Check,
    Checkmate,
    Stalmate,
    Playing
}