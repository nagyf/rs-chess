use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::Board;
use crate::engine::board::piece::color::Color;
use crate::engine::board::piece::general::square_attacked_by;
use crate::engine::board::piece::Piece;
use crate::engine::board::square::{File, Rank, Square};

// Test with an empty board
#[test]
fn square_attacked_by_empty() {
    let square = Square::from_pos(Rank::A, File::First);
    let attackers = square_attacked_by(square, &Board::empty());
    assert_eq!(BitBoard::empty(), attackers);
}

//
// Test with 1 queen, 1 bishop, 1 rook
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// R x x s . . . .
// . . . x x . . .
// . . . x . x . .
// . . . x . . x .
// . . . Q . . . b
//
#[test]
fn square_attacked_by_queen_bishop_rook() {
    let mut board = Board::empty();
    board.xor(Piece::Queen,
              Color::White,
              Square::from_pos(Rank::A, File::Fourth).as_bb());
    board.xor(Piece::Bishop,
              Color::Black,
              Square::from_pos(Rank::A, File::Eighth).as_bb());
    board.xor(Piece::Rook,
              Color::White,
              Square::from_pos(Rank::E, File::First).as_bb());

    let square = Square::from_pos(Rank::E, File::Fourth);
    let attackers = square_attacked_by(square, &board);
    assert_eq!(BitBoard::from(0x0000000100000088), attackers);
}

//
// Test with 1 queen, 1 bishop, 1 rook and 1 king
//
// . . . . . . . .
// . . . . . . . .
// . . . . K . . .
// R x x s . . . .
// . . . x x . . .
// . . . x . x . .
// . . . x . . x .
// . . . Q . . . b
//
#[test]
fn square_attacked_by_queen_bishop_rook_king() {
    let mut board = Board::empty();
    board.xor(Piece::Queen,
              Color::White,
              Square::from_pos(Rank::A, File::Fourth).as_bb());
    board.xor(Piece::Bishop,
              Color::Black,
              Square::from_pos(Rank::A, File::Eighth).as_bb());
    board.xor(Piece::Rook,
              Color::White,
              Square::from_pos(Rank::E, File::First).as_bb());
    board.xor(Piece::King,
              Color::White,
              Square::from_pos(Rank::F, File::Fifth).as_bb());

    let square = Square::from_pos(Rank::E, File::Fourth);
    let attackers = square_attacked_by(square, &board);
    assert_eq!(BitBoard::from(0x0000100100000088), attackers);
}

//
// Check with blocked attackers
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// R x o s . . . .
// . . . o o . . .
// . . . x . x . .
// . . . x . . x .
// . . . Q . . . b
//
#[test]
fn square_attacked_by_queen_bishop_rook_king_occupied() {
    let mut board = Board::empty();
    board.xor(Piece::Queen,
              Color::White,
              Square::from_pos(Rank::A, File::Fourth).as_bb());
    board.xor(Piece::Bishop,
              Color::Black,
              Square::from_pos(Rank::A, File::Eighth).as_bb());
    board.xor(Piece::Rook,
              Color::White,
              Square::from_pos(Rank::E, File::First).as_bb());

    // Blocking pieces
    board.xor(Piece::Pawn,
              Color::Black,
              Square::from_pos(Rank::D, File::Fourth).as_bb());
    board.xor(Piece::Pawn,
              Color::Black,
              Square::from_pos(Rank::D, File::Fifth).as_bb());
    board.xor(Piece::Pawn,
              Color::Black,
              Square::from_pos(Rank::E, File::Third).as_bb());

    let square = Square::from_pos(Rank::E, File::Fourth);
    let attackers = square_attacked_by(square, &board);
    assert_eq!(BitBoard::empty(), attackers);
}

//
// Test with 1 knight
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . . s . . . .
// . . . p . . . .
// . . . . n . . .
// . . . . . . . .
// . . . . . . . .
//
#[test]
fn square_attacked_by_knight() {
    let mut board = Board::empty();
    board.xor(Piece::Knight,
              Color::White,
              Square::from_pos(Rank::C, File::Fifth).as_bb());
    board.xor(Piece::Pawn,
              Color::Black,
              Square::from_pos(Rank::D, File::Fourth).as_bb());

    let square = Square::from_pos(Rank::E, File::Fourth);
    let attackers = square_attacked_by(square, &board);
    assert_eq!(BitBoard::from(0x0000000000100000), attackers);
}

//
// Test with 4 pawns
//
// . . . . . . . .
// . . . . . . . .
// . . p . p . . .
// . . . s . . . .
// . . P . P . . .
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
//
#[test]
fn square_attacked_by_pawns() {
    let mut board = Board::empty();
    board.xor(Piece::Pawn,
              Color::White,
              Square::from_pos(Rank::D, File::Third).as_bb());
    board.xor(Piece::Pawn,
              Color::White,
              Square::from_pos(Rank::D, File::Fifth).as_bb());

    board.xor(Piece::Pawn,
              Color::Black,
              Square::from_pos(Rank::F, File::Third).as_bb());
    board.xor(Piece::Pawn,
              Color::Black,
              Square::from_pos(Rank::F, File::Fifth).as_bb());

    let square = Square::from_pos(Rank::E, File::Fourth);
    let attackers = square_attacked_by(square, &board);
    assert_eq!(BitBoard::from(0x0000140014000000), attackers);
}
