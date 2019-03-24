use std::fmt::{Display, Error, Formatter};

use crate::engine::board::bitboard::BitBoard;

pub mod constants;

#[cfg(test)]
mod tests;

/// Represents the rank (the rows) of the chessboard
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rank {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

/// Represents the file (the columns) of the chessboard
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum File {
    First = 1,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

/// Represents a square on the chessboard
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Square(u8);

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let rank = match *self {
            Rank::A => "a",
            Rank::B => "b",
            Rank::C => "c",
            Rank::D => "d",
            Rank::E => "e",
            Rank::F => "f",
            Rank::G => "g",
            Rank::H => "h",
        };

        write!(f, "{}", rank)
    }
}

impl Rank {
    /// Converts a string to a Rank. Only "a".."h" values are allowed.
    pub fn from_string(id: &str) -> Option<Rank> {
        match id.to_lowercase().trim().as_ref() {
            "a" => Some(Rank::A),
            "b" => Some(Rank::B),
            "c" => Some(Rank::C),
            "d" => Some(Rank::D),
            "e" => Some(Rank::E),
            "f" => Some(Rank::F),
            "g" => Some(Rank::G),
            "h" => Some(Rank::H),
            _ => None
        }
    }

    /// Converts an integer to a Rank, only 1..8 values are allowed.
    pub fn from_index(index: u8) -> Option<Rank> {
        match index {
            1 => Some(Rank::A),
            2 => Some(Rank::B),
            3 => Some(Rank::C),
            4 => Some(Rank::D),
            5 => Some(Rank::E),
            6 => Some(Rank::F),
            7 => Some(Rank::G),
            8 => Some(Rank::H),
            _ => None
        }
    }

    /// Converts the Rank to integer, values returned are in the 1..8 range.
    pub fn to_index(&self) -> u8 {
        *self as u8
    }
}

impl File {
    /// Converts an integer to a File, only values 1..8 are allowed.
    pub fn from_index(index: u8) -> Option<File> {
        match index {
            1 => Some(File::First),
            2 => Some(File::Second),
            3 => Some(File::Third),
            4 => Some(File::Fourth),
            5 => Some(File::Fifth),
            6 => Some(File::Sixth),
            7 => Some(File::Seventh),
            8 => Some(File::Eighth),
            _ => None
        }
    }

    /// Converts the File to integer, values returned are in the 1..8 range.
    pub fn to_index(&self) -> u8 {
        *self as u8
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let file = match *self {
            File::First => "1",
            File::Second => "2",
            File::Third => "3",
            File::Fourth => "4",
            File::Fifth => "5",
            File::Sixth => "6",
            File::Seventh => "7",
            File::Eighth => "8",
        };

        write!(f, "{}", file)
    }
}

impl Default for Square {
    fn default() -> Self {
        Square::new(0)
    }
}

impl Square {
    /// Creates a new index using the specified raw value. Accepts values between 0..63
    pub fn new(index: u8) -> Square {
        if index > 63 {
            panic!(format!("Invalid square value: {}!", index))
        }

        Square(index)
    }

    /// Returns a square for the specified rank and file
    pub fn from_pos(rank: Rank, file: File) -> Square {
        let rank = rank.to_index() - 1;
        let file = file.to_index() - 1;
        Square::new(rank * 8 + file)
    }

    /// Converts a Bitboard to a square.
    pub fn from_bb(bb: BitBoard) -> Square {
        // This function makes no sense if it is called for a bitboard with multiple bits toggled
        debug_assert!(bb.0 % 2 == 0 || bb.0 == 1);

        let value_index = bb.bit_scan_fw();
        let rank: u8 = (value_index / 8) as u8 + 1;
        let file: u8 = (value_index % 8) as u8 + 1;
        Square::from_pos(Rank::from_index(rank).unwrap(),
                         File::from_index(file).unwrap())
    }

    /// Returns the raw value, the index of the square. Can be a value between 0..63
    pub fn to_index(&self) -> u64 {
        self.0 as u64
    }

    /// Converts the square to a bitboard in which the only 1 bit will be the square itself
    pub fn as_bb(&self) -> BitBoard {
        BitBoard::from(1u64 << self.to_index())
    }

    /// Returns the rank of this square
    pub fn get_rank(&self) -> Rank {
        Rank::from_index(self.0 / 8 + 1).unwrap()
    }

    /// Returns the file of this square
    pub fn get_file(&self) -> File {
        File::from_index(self.0 % 8 + 1).unwrap()
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}{}", self.get_rank(), self.get_file())
    }
}
