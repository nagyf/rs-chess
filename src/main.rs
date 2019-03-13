use crate::engine::board::Board;

pub mod engine;

fn main() {
    let board = Board::new();
    println!("{}", board);
}
