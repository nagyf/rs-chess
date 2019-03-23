use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::Board;
use crate::engine::board::piece::{Piece};
use crate::engine::board::piece::color::Color;
use crate::engine::board::piece::castling::CastlingRight;

//
// Empty board
//

#[test]
fn empty_moves() {
    let board = Board::empty();
    assert_eq!(0, board.half_moves);
    assert_eq!(0, board.full_moves);
}

#[test]
fn empty_colors() {
    let board = Board::empty();
    assert_eq!(2, board.colors.len());
    // Check colors
    assert_eq!(BitBoard::new(), board.colors[Color::White.to_index()]);
    assert_eq!(BitBoard::new(), board.colors[Color::Black.to_index()]);
}

#[test]
fn empty_pieces() {
    let board = Board::empty();
    // Check pieces
    assert_eq!(BitBoard::new(), board.pieces[Piece::Pawn.to_index()]);
    assert_eq!(BitBoard::new(), board.pieces[Piece::Rook.to_index()]);
    assert_eq!(BitBoard::new(), board.pieces[Piece::Knight.to_index()]);
    assert_eq!(BitBoard::new(), board.pieces[Piece::Bishop.to_index()]);
    assert_eq!(BitBoard::new(), board.pieces[Piece::King.to_index()]);
    assert_eq!(BitBoard::new(), board.pieces[Piece::Queen.to_index()]);
}

#[test]
fn empty_en_passant() {
    let board = Board::empty();
    assert_eq!(None, board.en_passant);
}

#[test]
fn empty_castling() {
    let board = Board::empty();
    assert_eq!([CastlingRight::NoRight, CastlingRight::NoRight], board.castling_rights);
}

#[test]
fn empty_turn() {
    let board = Board::empty();
    assert_eq!(Color::White, board.turn);
}

//
// Initial board
//

#[test]
fn initial_half_moves() {
    let board = Board::new();
    assert_eq!(0, board.half_moves);
}

#[test]
fn initial_full_moves() {
    let board = Board::new();
    assert_eq!(1, board.full_moves);
}

#[test]
fn initial_colors() {
    let board = Board::new();
    assert_eq!(2, board.colors.len());
    // Check colors
    assert_eq!(Color::White.initial_position(), board.colors[Color::White.to_index()]);
    assert_eq!(Color::Black.initial_position(), board.colors[Color::Black.to_index()]);
}

#[test]
fn initial_pieces() {
    let board = Board::new();
    // Check pieces
    assert_eq!(Piece::Pawn.initial_position(), board.pieces[Piece::Pawn.to_index()]);
    assert_eq!(Piece::Rook.initial_position(), board.pieces[Piece::Rook.to_index()]);
    assert_eq!(Piece::Knight.initial_position(), board.pieces[Piece::Knight.to_index()]);
    assert_eq!(Piece::Bishop.initial_position(), board.pieces[Piece::Bishop.to_index()]);
    assert_eq!(Piece::King.initial_position(), board.pieces[Piece::King.to_index()]);
    assert_eq!(Piece::Queen.initial_position(), board.pieces[Piece::Queen.to_index()]);
}

#[test]
fn initial_en_passant() {
    let board = Board::new();
    assert_eq!(None, board.en_passant);
}

#[test]
fn initial_castling() {
    let board = Board::new();
    assert_eq!([CastlingRight::BothSide, CastlingRight::BothSide], board.castling_rights);
}

#[test]
fn initial_turn() {
    let board = Board::new();
    assert_eq!(Color::White, board.turn);
}

//
// Other methods
//

