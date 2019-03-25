//! This module implements the `Color` enum that is used to distinguish the 2 players' pieces.

use crate::engine::board::bitboard::BitBoard;
use std::fmt::{Display, Formatter, Error};
use std::ops::Not;

/// Represents a color to be able to distinguish the 2 players' pieces.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Color {
    /// The white player, who opens the game.
    White = 0,
    /// The black player, who is the second to move.
    Black = 1,
}

/// An array containing all colors.
pub const ALL_COLORS: [Color; NUM_COLORS] = [Color::White, Color::Black];

/// The number of colors (2).
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
    /// Converts the color to an index represented by the enum, so it can be used to index arrays.
    ///
    /// `White` will be `0` and `Black` will be `1`.
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Returns a bitboard with the initial positions for the color.
    ///
    /// # Example
    ///
    /// ```
    /// println!("{}", Color::White.initial_position())
    ///
    /// . . . . . . . .
    /// . . . . . . . .
    /// . . . . . . . .
    /// . . . . . . . .
    /// . . . . . . . .
    /// . . . . . . . .
    /// X X X X X X X X
    /// X X X X X X X X
    /// ```
    pub fn initial_position(&self) -> BitBoard {
        match *self {
            Color::Black => BitBoard::from(0xFFFF000000000000),
            Color::White => BitBoard::from(0x000000000000FFFF),
        }
    }
}

impl Not for Color {
    type Output = Color;

    /// Transposes the color, `White` becomes `Black`, `Black` becomes `White`.
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
