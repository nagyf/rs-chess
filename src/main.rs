use crate::engine::board::BitBoard;

pub mod engine;

fn main() {
    println!("{}", BitBoard::from(0x0000000000000080))
}
