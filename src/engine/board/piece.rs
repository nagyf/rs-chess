use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Type {
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
    WhiteQueen,
    WhiteKing,
    BlackQueen,
    BlackKing
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let color = match *self {
            Color::Black => "b",
            Color::White => "w",
        };

        write!(f, "{}", color)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let piece = match *self {
            Type::Pawn => "P",
            Type::Rook => "R",
            Type::Knight => "N",
            Type::Bishop => "B",
            Type::King => "K",
            Type::Queen => "Q",
        };

        write!(f, "{}", piece)
    }
}