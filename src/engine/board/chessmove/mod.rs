use crate::engine::board::piece::Piece;
use crate::engine::board::square::Square;

#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ChessMove {
    src: Square,
    dst: Square,
    promotion: Option<Piece>,
}

impl ChessMove {
    pub fn new(src: Square, dst: Square) -> ChessMove {
        ChessMove {
            src,
            dst,
            promotion: None,
        }
    }

    pub fn new_promote(src: Square, dst: Square, piece: Piece) -> ChessMove {
        ChessMove {
            src,
            dst,
            promotion: Some(piece),
        }
    }

    pub fn get_source(&self) -> Square {
        self.src
    }

    pub fn get_destination(&self) -> Square {
        self.dst
    }

    pub fn get_promotion(&self) -> Option<Piece> {
        self.promotion
    }
}
