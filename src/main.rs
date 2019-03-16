extern crate regex;

use crate::engine::board::Board;
use crate::engine::board::square::{Square, Rank, File};
use crate::engine::board::chessmove::ChessMove;

pub mod engine;

fn main() {
    let board = Board::new().make_move(
        ChessMove::new(
            Square::from_pos(Rank::B, File::Second),
            Square::from_pos(Rank::C, File::Second)
        )).unwrap().make_move(
        ChessMove::new(
            Square::from_pos(Rank::G, File::Fourth),
            Square::from_pos(Rank::F, File::Fourth)
        )).unwrap();

    println!("{}", board);
}
