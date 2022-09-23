use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

fn main() {
    
}

fn check_allowed_moves(board: [[Piece; 8]; 8], piece: (i8, i8)) -> Vec<(i8, i8)> {

    

    vec![(5, 5)] // Placeholder
}

fn check_vertical_and_horizontal(board: [[Piece; 8]; 8], position: (i8, i8), color: Color, limit: bool) -> Vec<(i8, i8)> {
    let mut allowed_moves: Vec<(i8, i8)> = Vec::new();

    let mut right: bool = false;
    let mut left: bool = false;
    let mut up: bool = false;
    let mut down: bool = false;

    let steps: i8 = if limit {1} else {7};

    let (x, y) = position;
    
    for n in 1..steps {
        if right && left && up && down {
            break;
        }

        if !right {
            let new_x: i8 = x + n;
            if new_x >= 0 && new_x <= 7{ 
                let new_position: Piece = board[new_x as usize][y as usize];
                if new_position.piece_type == PieceType::EMPTY {
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
                if new_position.piece_type == PieceType::EMPTY {
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
                if new_position.piece_type == PieceType::EMPTY {
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
                if new_position.piece_type == PieceType::EMPTY {
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

fn check_diagonal(board: [[Piece; 8]; 8], position: (i8, i8), color: Color, limit: bool) -> Vec<(i8, i8)> {
    let mut allowed_moves: Vec<(i8, i8)> = Vec::new();

    let mut stop_down_left: bool = false;
    let mut stop_down_right: bool = false;
    let mut stop_up_left: bool = false;
    let mut stop_up_right: bool = false;

    let steps: i8 = if limit {1} else {7};

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
                if new_position.piece_type == PieceType::EMPTY {
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
                if new_position.piece_type == PieceType::EMPTY {
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
                if new_position.piece_type == PieceType::EMPTY {
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
                if new_position.piece_type == PieceType::EMPTY {
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

fn create_board() -> [[Piece; 8]; 8] {
    // crates an 8x8 array of empt pieces
    let empty_piece: Piece = Piece {
        piece_type: PieceType::EMPTY,
        color: Color::WHITE
    };
    let mut board: [[Piece; 8]; 8] = [[empty_piece; 8]; 8];

    // loop through x positions and piecies
    for (position, piece) in (0..5).zip(PieceType::iter()) {
        board[position][0] = Piece { piece_type: piece, color: Color::WHITE };
        board[position][1] = Piece { piece_type: PieceType::PAWN, color: Color::WHITE };

        board[position][7] = Piece { piece_type: piece, color: Color::BLACK };
        board[position][6] = Piece { piece_type: PieceType::PAWN, color: Color::BLACK };
    }
    for (position, piece) in (8..5).zip(PieceType::iter()) {
        board[position][0] = Piece { piece_type: piece, color: Color::WHITE };
        board[position][1] = Piece { piece_type: PieceType::PAWN, color: Color::WHITE };

        board[position][7] = Piece { piece_type: piece, color: Color::BLACK };
        board[position][6] = Piece { piece_type: PieceType::PAWN, color: Color::BLACK };
    }

    board
}

#[derive(Copy, Clone, Debug)]
struct Piece {
    pub piece_type: PieceType,
    pub color: Color
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    WHITE,
    BLACK
}

#[derive(Copy, Clone, Debug, EnumIter, PartialEq)]
enum PieceType {
    ROOK,
    KNIGHT,
    BISHOP,
    KING,
    QUEEN,
    PAWN,
    EMPTY
}