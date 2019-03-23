use crate::engine::board::piece::Piece;
use crate::engine::board::square::{Rank, File, Square};
use crate::engine::fen;
use crate::engine::board::piece::color::Color;
use crate::engine::board::piece::castling::CastlingRight;

#[test]
fn parse_half_moves() {
    let result = super::parse_half_moves("10").unwrap();
    assert_eq!(10, result);
}

#[test]
#[should_panic]
fn parse_half_moves_error() {
    super::parse_half_moves("abc").unwrap();
}

#[test]
fn parse_full_moves() {
    let result = super::parse_full_moves("10").unwrap();
    assert_eq!(10, result);
}

#[test]
#[should_panic]
fn parse_full_moves_error() {
    super::parse_full_moves("abc").unwrap();
}

#[test]
fn parse_side_to_move() {
    assert_eq!(Color::White, super::parse_side_to_move("w").unwrap());
    assert_eq!(Color::Black, super::parse_side_to_move("b").unwrap());
}

#[test]
#[should_panic]
fn parse_side_to_move_error() {
    super::parse_side_to_move("z").unwrap();
}

#[test]
fn parse_castling_rights() {
    let rights = super::parse_castling_rights("KkQq").unwrap();
    assert_eq!([CastlingRight::BothSide, CastlingRight::BothSide], rights);
}

#[test]
fn parse_castling_rights_black() {
    let rights = super::parse_castling_rights("kq").unwrap();
    assert_eq!([CastlingRight::NoRight, CastlingRight::BothSide], rights);
}

#[test]
fn parse_castling_rights_white() {
    let rights = super::parse_castling_rights("KQ").unwrap();
    assert_eq!([CastlingRight::BothSide, CastlingRight::NoRight], rights);
}

#[test]
fn parse_castling_rights_empty() {
    let rights = super::parse_castling_rights("-").unwrap();
    assert_eq!(2, rights.len());
    assert_eq!([CastlingRight::NoRight, CastlingRight::NoRight], rights);
}

#[test]
#[should_panic]
fn parse_castling_rights_error() {
    super::parse_side_to_move("z").unwrap();
}

#[test]
fn parse_en_passant() {
    let result = super::parse_en_passant("e6").unwrap();
    assert_eq!(Some(Square::from_pos(Rank::E, File::Sixth)), result);
}

#[test]
fn parse_en_passant_empty() {
    let result = super::parse_en_passant("-").unwrap();
    assert_eq!(None, result);
}

#[test]
#[should_panic]
fn parse_en_passant_error() {
    super::parse_en_passant("z").unwrap();
}

#[test]
fn parse_pieces_empty() {
    let result = super::parse_pieces("8/8/8/8/8/8/8/8").unwrap();
    assert_eq!(0, result.len());
}

#[test]
fn parse_pieces_starting_position() {
    let result = super::parse_pieces("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap();
    assert_eq!(32, result.len());
}

#[test]
fn parse_pieces_singleton() {
    let result = super::parse_pieces("7k/8/8/8/8/8/8/8").unwrap();
    assert_eq!(1, result.len());
    assert_eq!(Rank::H, result[0].square.get_rank());
    assert_eq!(File::Eighth, result[0].square.get_file());
}

#[test]
#[should_panic]
fn parse_pieces_error() {
    super::parse_pieces("7k/8/8/8/invalid/8/8/8").unwrap();
}

#[test]
fn from_fen() {
    let board = super::from_fen(fen::INITIAL_BOARD).unwrap();
    assert_eq!(0, board.get_half_moves());
    assert_eq!(1, board.get_full_moves());
    assert_eq!(Color::White, board.get_turn());
    assert_eq!([CastlingRight::BothSide, CastlingRight::BothSide], board.get_castling_rights());
    assert_eq!(None, board.get_en_passant());
    assert_eq!(Piece::Pawn.initial_position(), board.get_pieces(Piece::Pawn));
    assert_eq!(Piece::Rook.initial_position(), board.get_pieces(Piece::Rook));
    assert_eq!(Piece::Knight.initial_position(), board.get_pieces(Piece::Knight));
    assert_eq!(Piece::Bishop.initial_position(), board.get_pieces(Piece::Bishop));
    assert_eq!(Piece::King.initial_position(), board.get_pieces(Piece::King));
    assert_eq!(Piece::Queen.initial_position(), board.get_pieces(Piece::Queen));
}

#[test]
#[should_panic]
fn from_fen_error() {
    super::from_fen("invalid/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
}