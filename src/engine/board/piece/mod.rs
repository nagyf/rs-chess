use std::fmt::{Display, Error, Formatter};

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
    Black = 0,
    White = 1,
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
pub const ALL_COLORS: [Color; NUM_COLORS] = [Color::Black, Color::White];
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
}

impl Piece {
    pub fn to_index(&self) -> usize {
        *self as usize
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