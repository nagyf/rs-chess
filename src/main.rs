use crate::engine::board::BitBoard;

pub mod engine;

fn main() {
    let board = BitBoard::from(0x000000000000000F);
    println!("{}", board);
    println!("{}", board.rotate_90_cw())
}
