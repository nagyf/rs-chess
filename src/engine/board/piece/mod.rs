use std::fmt::{Display, Error, Formatter};
use std::ops::Not;

use crate::engine::board::bitboard::BitBoard;

pub mod pawn;
pub mod knight;
pub mod king;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CastlingRight {
    QueenSide,
    KingSide,
    BothSide,
    NoRight,
}

pub const NUM_PIECES: usize = 6;
pub const NUM_COLORS: usize = 2;
pub const ALL_COLORS: [Color; NUM_COLORS] = [Color::White, Color::Black];
pub const ALL_PIECES: [Piece; NUM_PIECES] = [
    Piece::Pawn,
    Piece::Rook,
    Piece::Knight,
    Piece::Bishop,
    Piece::King,
    Piece::Queen
];

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let color = match *self {
            Color::Black => "b",
            Color::White => "w",
        };

        write!(f, "{}", color)
    }
}

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

impl Color {
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn initial_position(&self) -> BitBoard {
        match *self {
            Color::Black => BitBoard::from(0xFFFF000000000000),
            Color::White => BitBoard::from(0x000000000000FFFF),
        }
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Piece {
    pub fn to_index(&self) -> usize {
        *self as usize
    }

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

impl CastlingRight {
    pub fn merge(&self, other: CastlingRight) -> CastlingRight {
        match *self {
            CastlingRight::NoRight => other,
            CastlingRight::BothSide => *self,
            _ => {
                if *self == other {
                    other
                } else {
                    match other {
                        CastlingRight::QueenSide | CastlingRight::KingSide => CastlingRight::BothSide,
                        CastlingRight::NoRight => *self,
                        CastlingRight::BothSide => CastlingRight::BothSide
                    }
                }
            }
        }
    }
}