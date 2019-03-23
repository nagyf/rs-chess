use crate::engine::board::chessmove::ChessMove;
use crate::engine::board::square::{Square, Rank, File};

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