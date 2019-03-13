extern crate regex;

use crate::engine::board::Board;
use crate::engine::fen;
use crate::engine::fen::FENParseError;

pub mod engine;

fn main() -> Result<(), FENParseError> {
    let board = fen::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2")?;
    println!("{:?}", board);
    println!("{}", board);
    Ok(())
}
