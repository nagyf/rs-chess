use std::fmt::{Display, Error, Formatter};

use crate::engine::board::bitboard::BitBoard;

pub mod sliding;
pub mod pawn;
pub mod knight;
pub mod king;
pub mod color;
pub mod castling;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Piece {
    Pawn = 0,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

pub const NUM_PIECES: usize = 6;

pub const ALL_PIECES: [Piece; NUM_PIECES] = [
    Piece::Pawn,
    Piece::Rook,
    Piece::Knight,
    Piece::Bishop,
    Piece::King,
    Piece::Queen
];

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let piece = match *self {
            Piece::Pawn => "P",
            Piece::Rook => "R",
            Piece::Knight => "N",
            Piece::Bishop => "B",
            Piece::King => "K",
            Piece::Queen => "Q",
        };

        write!(f, "{}", piece)
    }
}


impl Piece {
    /// Converts the piece type to an index, so it can be used to index arrays
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Returns a bitboard with the initial positions for the current piece
    pub fn initial_position(&self) -> BitBoard {
        match *self {
            Piece::Pawn => Piece::pawns_initial(),
            Piece::Rook => Piece::rooks_initial(),
            Piece::Knight => Piece::knights_initial(),
            Piece::Bishop => Piece::bishops_initial(),
            Piece::King => Piece::kings_initial(),
            Piece::Queen => Piece::queens_initial(),
        }
    }

    /// Returns the bitboard representing the starting positions of pawns
    fn pawns_initial() -> BitBoard {
        BitBoard::from(0x00FF00000000FF00)
    }

    /// Returns the bitboard representing the starting positions of rooks
    fn rooks_initial() -> BitBoard {
        BitBoard::from(0x8100000000000081)
    }

    /// Returns the bitboard representing the starting positions of knights
    fn knights_initial() -> BitBoard {
        BitBoard::from(0x4200000000000042)
    }

    /// Returns the bitboard representing the starting positions of bishops
    fn bishops_initial() -> BitBoard {
        BitBoard::from(0x2400000000000024)
    }

    /// Returns the bitboard representing the starting positions of kings
    fn kings_initial() -> BitBoard {
        BitBoard::from(0x1000000000000010)
    }

    /// Returns the bitboard representing the starting positions of queens
    fn queens_initial() -> BitBoard {
        BitBoard::from(0x0800000000000008)
    }
}
