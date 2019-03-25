use crate::engine::board::chessmove::ChessMove;
use crate::engine::board::piece::Piece;
use crate::engine::board::square::{File, Rank, Square};

#[test]
fn new() {
    let from = Square::from_pos(Rank::A, File::First);
    let to = Square::from_pos(Rank::B, File::Second);
    let chess_move = ChessMove::new(from, to);
    assert_eq!(from, chess_move.src);
    assert_eq!(to, chess_move.dst);
    assert_eq!(None, chess_move.promotion);
}

#[test]
fn new_promote() {
    let from = Square::from_pos(Rank::A, File::First);
    let to = Square::from_pos(Rank::B, File::Second);
    let promote = Piece::Queen;
    let chess_move = ChessMove::new_promote(from, to, promote);
    assert_eq!(from, chess_move.src);
    assert_eq!(to, chess_move.dst);
    assert_eq!(Some(promote), chess_move.promotion);
}

#[test]
fn get_source() {
    let from = Square::from_pos(Rank::A, File::First);
    let to = Square::from_pos(Rank::B, File::Second);
    let chess_move = ChessMove::new(from, to);
    assert_eq!(from, chess_move.get_source());
}

#[test]
fn get_destination() {
    let from = Square::from_pos(Rank::A, File::First);
    let to = Square::from_pos(Rank::B, File::Second);
    let chess_move = ChessMove::new(from, to);
    assert_eq!(to, chess_move.get_destination());
}

#[test]
fn get_promotion() {
    let from = Square::from_pos(Rank::A, File::First);
    let to = Square::from_pos(Rank::B, File::Second);
    let chess_move = ChessMove::new_promote(from, to, Piece::Queen);
    assert_ne!(None, chess_move.get_promotion());
    assert_eq!(Piece::Queen, chess_move.get_promotion().unwrap());
}
