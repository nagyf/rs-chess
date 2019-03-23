use crate::engine::board::bitboard::BitBoard;
use std::fmt::{Display, Formatter, Error};
use std::ops::Not;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Color {
    White = 0,
    Black = 1,
}

pub const ALL_COLORS: [Color; NUM_COLORS] = [Color::White, Color::Black];
pub const NUM_COLORS: usize = 2;

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let color = match *self {
            Color::Black => "b",
            Color::White => "w",
        };

        write!(f, "{}", color)
    }
}

impl Color {
    /// Converts the color to an index represented by the enum, so it can be used to index arrays
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Returns a bitboard with the initial positions for the color
    pub fn initial_position(&self) -> BitBoard {
        match *self {
            Color::Black => BitBoard::from(0xFFFF000000000000),
            Color::White => BitBoard::from(0x000000000000FFFF),
        }
    }
}

impl Not for Color {
    type Output = Color;

    /// Transposes the color, white becomes black, black becomes white
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}