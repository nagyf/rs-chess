extern crate regex;

use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::piece::king;
use crate::engine::board::square::{File, Rank};
use crate::engine::fen::FENParseError;

pub mod engine;

fn main() -> Result<(), FENParseError> {
    let board = BitBoard::empty().set(Rank::D, File::Fourth);
    println!("{}", king::attack_targets(board));
    Ok(())
}
