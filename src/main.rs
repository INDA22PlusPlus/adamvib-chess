use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

fn main() {

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
    piece_type: PieceType,
    color: Color
}

#[derive(Copy, Clone, Debug)]
enum Color {
    WHITE,
    BLACK
}

#[derive(Copy, Clone, Debug, EnumIter)]
enum PieceType {
    ROOK,
    KNIGHT,
    BISHOP,
    KING,
    QUEEN,
    PAWN,
    EMPTY
}