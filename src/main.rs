fn main() {
    // crates an 8x8 array of zeros
    let mut board = [[0u8; 8]; 8];
}

enum Color {
    White,
    Black
}

enum Piece {
    King(Color),
    Queen(Color),
    Knight(Color),
    Rook(Color),
    Bishop(Color),
    Pawn(Color)
}