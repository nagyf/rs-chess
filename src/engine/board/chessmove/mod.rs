//! This module implements `ChessMove`, which represents a move in chess.
//!
//! The move is represented by the *source* and the *destination* squares,
//! and optionally a promotion.

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
    /// Creates a new chess move.
    pub fn new(src: Square, dst: Square) -> ChessMove {
        ChessMove {
            src,
            dst,
            promotion: None,
        }
    }

    /// Creates a new chess move with a promotion.
    pub fn new_promote(src: Square, dst: Square, piece: Piece) -> ChessMove {
        ChessMove {
            src,
            dst,
            promotion: Some(piece),
        }
    }

    /// Returns the source square.
    pub fn get_source(&self) -> Square {
        self.src
    }

    /// Returns the destination square.
    pub fn get_destination(&self) -> Square {
        self.dst
    }

    /// Returns the promoted piece.
    pub fn get_promotion(&self) -> Option<Piece> {
        self.promotion
    }
}
