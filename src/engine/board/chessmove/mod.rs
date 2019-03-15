use crate::engine::board::square::Square;

#[derive(Debug, Eq, PartialEq)]
pub struct ChessMove {
    src: Square,
    dst: Square
}

impl ChessMove {
    pub fn new(src: Square, dst: Square) -> ChessMove{
        ChessMove {
            src,
            dst
        }
    }

    pub fn get_source(&self) -> Square {
        self.src
    }

    pub fn get_destination(&self) -> Square {
        self.dst
    }
}