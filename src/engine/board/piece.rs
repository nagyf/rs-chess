use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Color {
    Black,
    White
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CastlingRight {
    WhiteQueenSide,
    WhiteKingSide,
    BlackQueenSide,
    BlackKingSide
}

pub const NUM_PIECES: usize = 6;

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