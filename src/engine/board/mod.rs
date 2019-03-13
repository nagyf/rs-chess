use std::collections::HashMap;

use crate::engine::board::bitboard::BitBoard;
use std::fmt::{Display, Error, Formatter};

mod constants;
pub mod bitboard;
pub mod piece;

#[cfg(test)]
mod tests;

pub struct Board {
    turn: piece::Color,
    half_moves: u16,
    white: BitBoard,
    black: BitBoard,
    pieces: HashMap<piece::Type, BitBoard>,
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = HashMap::new();
        pieces.insert(piece::Type::Pawn, Board::pawns());
        pieces.insert(piece::Type::Rook, Board::rooks());
        pieces.insert(piece::Type::Knight, Board::knights());
        pieces.insert(piece::Type::Bishop, Board::bishops());
        pieces.insert(piece::Type::King, Board::kings());
        pieces.insert(piece::Type::Queen, Board::queens());

        Board {
            turn: piece::Color::White,
            half_moves: 0,
            white: BitBoard::from(0x000000000000FFFF),
            black: BitBoard::from(0xFFFF000000000000),
            pieces,
        }
    }

    /// Returns the bitboard representing the starting positions of pawns
    fn pawns() -> BitBoard {
        BitBoard::from(0x00FF00000000FF00)
    }

    /// Returns the bitboard representing the starting positions of rooks
    fn rooks() -> BitBoard {
        BitBoard::from(0x8100000000000081)
    }

    /// Returns the bitboard representing the starting positions of knights
    fn knights() -> BitBoard {
        BitBoard::from(0x4200000000000042)
    }

    /// Returns the bitboard representing the starting positions of bishops
    fn bishops() -> BitBoard {
        BitBoard::from(0x2400000000000024)
    }

    /// Returns the bitboard representing the starting positions of kings
    fn kings() -> BitBoard {
        BitBoard::from(0x1000000000000010)
    }

    /// Returns the bitboard representing the starting positions of queens
    fn queens() -> BitBoard {
        BitBoard::from(0x0800000000000008)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let board = self.white | self.black;

        write!(f, "{}", board)
    }
}