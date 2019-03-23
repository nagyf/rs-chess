use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::constants::{ANTIDIAGONAL_H1_A8, DIAGONAL_A1_H8};
use crate::engine::board::piece::Piece;
use crate::engine::board::piece::sliding::{bishop_attacks, get_piece_attacks, queen_attacks, rook_attacks};
use crate::engine::board::square::{constants, File, Rank, Square};

#[test]
fn get_piece_attacks_rook() {
    let square = Square::from_pos(Rank::D, File::Fourth);
    let occupied = BitBoard::empty();
    let attacks = get_piece_attacks(Piece::Rook,
                                    square,
                                    occupied);
    let expected = rook_attacks(square, occupied);
    assert_eq!(expected, attacks);
}

#[test]
fn get_piece_attacks_bishop() {
    let square = Square::from_pos(Rank::D, File::Fourth);
    let occupied = BitBoard::empty();
    let attacks = get_piece_attacks(Piece::Bishop,
                                    square,
                                    occupied);
    let expected = bishop_attacks(square, occupied);
    assert_eq!(expected, attacks);
}

#[test]
fn get_piece_attacks_queen() {
    let square = Square::from_pos(Rank::D, File::Fourth);
    let occupied = BitBoard::empty();
    let attacks = get_piece_attacks(Piece::Queen,
                                    square,
                                    occupied);
    let expected = queen_attacks(square, occupied);
    assert_eq!(expected, attacks);
}

#[test]
#[should_panic]
fn get_piece_attacks_other() {
    let square = Square::from_pos(Rank::D, File::Fourth);
    let occupied = BitBoard::empty();
    get_piece_attacks(Piece::Knight,
                      square,
                      occupied);
}


//
// Test with 1 rook in the corner
//
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// R x x x x x x x
//
#[test]
fn rook_corner_empty() {
    let rook = Square::from_pos(Rank::A, File::First);
    let attacks = rook_attacks(rook, BitBoard::empty());
    let expected = (constants::FILE_1 | constants::RANK_A) ^ rook.as_bb();
    assert_eq!(expected, attacks);
}

//
// Test with 1 rook in the middle
//
// . . . x . . . .
// . . . x . . . .
// . . . x . . . .
// . . . x . . . .
// x x x R x x x x
// . . . x . . . .
// . . . x . . . .
// . . . x . . . .
//
#[test]
fn rook_middle_empty() {
    let rook = Square::from_pos(Rank::D, File::Fourth);
    let attacks = rook_attacks(rook, BitBoard::empty());
    let expected = (constants::FILE_4 | constants::RANK_D) ^ rook.as_bb();
    assert_eq!(expected, attacks);
}

//
// Test with 1 rook in the corner, with 1 square occupied in the vertical line
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// o . . . . . . .
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// R x x x x x x x
//
#[test]
fn rook_corner_occupied_vertical() {
    let rook = Square::from_pos(Rank::A, File::First);
    let occupied = Square::from_pos(Rank::E, File::First).as_bb();
    let attacks = rook_attacks(rook, occupied);
    let expected = (constants::FILE_1 | constants::RANK_A) ^ rook.as_bb() ^ BitBoard::from(0x0101010000000000);
    assert_eq!(expected, attacks);
}

//
// Test with 1 rook in the corner, with 1 square occupied in the horizontal line
//
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// x . . . . . . .
// R x x x o . . .
//
#[test]
fn rook_corner_occupied_horizontal() {
    let rook = Square::from_pos(Rank::A, File::First);
    let occupied = Square::from_pos(Rank::A, File::Fifth).as_bb();
    let attacks = rook_attacks(rook, occupied);
    let expected = (constants::FILE_1 | constants::RANK_A) ^ rook.as_bb() ^ BitBoard::from(0x00000000000000E0);
    assert_eq!(expected, attacks);
}

//
// Test with 1 rook in the middle, with 1 square occupied in every line
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . . o . . . .
// . . o R o . . .
// . . . o . . . .
// . . . . . . . .
// . . . . . . . .
//
#[test]
fn rook_middle_occupied_all() {
    let rook = Square::from_pos(Rank::D, File::Fourth);
    let occupied = constants::FILE_3
        | constants::FILE_5
        | constants::RANK_C
        | constants::RANK_E;
    let attacks = rook_attacks(rook, occupied);
    let expected = BitBoard::from(0x0000000814080000);
    assert_eq!(expected, attacks);
}

//
// Test with 1 bishop in the corner
//
// . . . . . . . x
// . . . . . . x .
// . . . . . x . .
// . . . . x . . .
// . . . x . . . .
// . . x . . . . .
// . x . . . . . .
// B . . . . . . .
//
#[test]
fn bishop_corner_empty() {
    let bishop = Square::from_pos(Rank::A, File::First);
    let attacks = bishop_attacks(bishop, BitBoard::empty());
    let expected = DIAGONAL_A1_H8 ^ bishop.as_bb();
    assert_eq!(expected, attacks);
}

//
// Test with 1 bishop in the middle
//
// . . . . . . . x
// x . . . . . x .
// . x . . . x . .
// . . x . x . . .
// . . . B . . . .
// . . x . x . . .
// . x . . . x . .
// x . . . . . x .
//
#[test]
fn bishop_middle_empty() {
    let bishop = Square::from_pos(Rank::D, File::Fourth);
    let attacks = bishop_attacks(bishop, BitBoard::empty());
    let expected = (DIAGONAL_A1_H8 | ANTIDIAGONAL_H1_A8.south_one()) ^ bishop.as_bb();
    assert_eq!(expected, attacks);
}

//
// Test with 1 bishop in the corner, with 1 square occupied in the diagonal line
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . . . o . . .
// . . . x . . . .
// . . x . . . . .
// . x . . . . . .
// B . . . . . . .
//
#[test]
fn bishop_corner_occupied_diagonal() {
    let bishop = Square::from_pos(Rank::A, File::First);
    let occupied = Square::from_pos(Rank::E, File::Fifth).as_bb();
    let attacks = bishop_attacks(bishop, occupied);
    let expected = DIAGONAL_A1_H8 ^ bishop.as_bb() ^ BitBoard::from(0x8040200000000000);
    assert_eq!(expected, attacks);
}

//
// Test with 1 bishop in the corner, with 1 square occupied in the anti-diagonal line
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . . o . . . .
// . . . . x . . .
// . . . . . x . .
// . . . . . . x .
// . . . . . . . B
//
#[test]
fn bishop_corner_occupied_antidiagonal() {
    let bishop = Square::from_pos(Rank::A, File::Eighth);
    let occupied = Square::from_pos(Rank::E, File::Fourth).as_bb();
    let attacks = bishop_attacks(bishop, occupied);
    let expected = ANTIDIAGONAL_H1_A8 ^ bishop.as_bb() ^ BitBoard::from(0x0102040000000000);
    assert_eq!(expected, attacks);
}

//
// Test with 1 bishop in the middle, both diagonal occupied
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . o . o . . .
// . . . B . . . .
// . . o . o . . .
// . . . . . . . .
// . . . . . . . .
//
#[test]
fn bishop_middle_occupied() {
    let bishop = Square::from_pos(Rank::D, File::Fourth);
    let occupied = constants::FILE_3 | constants::FILE_5;
    let attacks = bishop_attacks(bishop, occupied);
    let expected = BitBoard::from(0x0000001400140000);
    assert_eq!(expected, attacks);
}

//
// Test with 1 queen in the corner
//
// x . . . . . . x
// x . . . . . x .
// x . . . . x . .
// x . . . x . . .
// x . . x . . . .
// x . x . . . . .
// x x . . . . . .
// Q x x x x x x x
//
#[test]
fn queen_corner_empty() {
    let queen = Square::from_pos(Rank::A, File::First);
    let attacks = queen_attacks(queen, BitBoard::empty());
    let expected = (DIAGONAL_A1_H8 | constants::RANK_A | constants::FILE_1) ^ queen.as_bb();
    assert_eq!(expected, attacks);
}

//
// Test with 1 queen in the middle
//
// . . . x . . . x
// x . . x . . x .
// . x . x . x . .
// . . x x x . . .
// x x x Q x x x x
// . . x x x . . .
// . x . x . x . .
// x . . x . . x .
//
#[test]
fn queen_middle_empty() {
    let queen = Square::from_pos(Rank::D, File::Fourth);
    let attacks = queen_attacks(queen, BitBoard::empty());
    let expected = (DIAGONAL_A1_H8
        | ANTIDIAGONAL_H1_A8.south_one()
        | constants::RANK_D
        | constants::FILE_4)
        ^ queen.as_bb();
    assert_eq!(expected, attacks);
}

//
// Test with 1 queen in the middle, occupied all the squares next to her
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . o o o . . .
// . . o Q o . . .
// . . o o o . . .
// . . . . . . . .
// . . . . . . . .
//
#[test]
fn queen_middle_occupied() {
    let queen = Square::from_pos(Rank::D, File::Fourth);
    let occupied = (constants::FILE_3
        | constants::FILE_5
        | constants::RANK_C
        | constants::RANK_D
        | constants::RANK_E)
        ^ queen.as_bb();

    let attacks = queen_attacks(queen, occupied);
    let expected = BitBoard::from(0x0000001C141C0000);

    assert_eq!(expected, attacks);
}
