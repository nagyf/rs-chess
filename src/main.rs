extern crate regex;

use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::Board;
use crate::engine::board::chessmove::ChessMove;
use crate::engine::board::piece::sliding;
use crate::engine::board::square::{File, Rank, Square};
use crate::engine::board::square::constants::{FILE_A, FILE_D, FILE_B};

pub mod engine;

fn main() {
    /*let board = Board::new().make_move(
        ChessMove::new(
            Square::from_pos(Rank::B, File::Second),
            Square::from_pos(Rank::D, File::Second)
        )).unwrap().make_move(
        ChessMove::new(
            Square::from_pos(Rank::G, File::Second),
            Square::from_pos(Rank::E, File::Second)
        )).unwrap().make_move(
        ChessMove::new(
            Square::from_pos(Rank::A, File::Second),
            Square::from_pos(Rank::C, File::Third)
        )).unwrap().make_move(
        ChessMove::new(
            Square::from_pos(Rank::G, File::Fourth),
            Square::from_pos(Rank::E, File::Fourth)
        )).unwrap().make_move(
        ChessMove::new(
            Square::from_pos(Rank::C, File::Third),
            Square::from_pos(Rank::E, File::Second)
        )).unwrap();

    println!("{}", board);*/
    println!("{}", sliding::queen_attacks(Square::from_pos(Rank::D, File::Fourth), FILE_B));
}
