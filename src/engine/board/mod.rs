use std::fmt::{Display, Error, Formatter};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

mod constants;
pub mod piece;

#[cfg(test)]
mod tests;

///
/// Bitboard implemented with Little endian rank-file (LERF) mapping
/// For more information: https://www.chessprogramming.org/Square_Mapping_Considerations#Little-Endian_Rank-File_Mapping
///
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct BitBoard(pub u64);

impl Default for BitBoard {
    fn default() -> Self {
        Self(0)
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut result: String = "".to_owned();
        let mut row: String = "".to_owned();
        for x in 0..64 {
            if self.0 & (1u64 << x) == (1u64 << x) {
                row.push_str("X ");
            } else {
                row.push_str(". ");
            }
            if x % 8 == 7 {
                row.push_str("\n");
                result.insert_str(0, &row);
                row.clear();
            }
        }
        write!(f, "{}", result)
    }
}

impl BitBoard {

    /// Creates an empty bitboard
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a bitboard from a u64 value
    pub fn from(value: u64) -> Self {
        BitBoard(value)
    }

    /// Creates an empty bitboard
    pub fn empty() -> Self {
        Self::new()
    }

    /// Creates a bitboard in which every bit is set to 1
    pub fn universe() -> Self {
        Self::from(constants::UNIVERSE)
    }
}

impl BitBoard {
    pub fn value(&self) -> u64 {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn empty_squares(&self) -> BitBoard {
        !(*self)
    }

    pub fn complement(&self) -> BitBoard {
        !*self
    }

    pub fn north_one(&self) -> BitBoard {
        *self << 8
    }

    pub fn south_one(&self) -> BitBoard {
        *self >> 8
    }

    pub fn east_one(&self) -> BitBoard {
        (*self << 1) & constants::NOT_A_FILE
    }

    pub fn no_ea_one(&self) -> BitBoard {
        (*self << 9) & constants::NOT_A_FILE
    }

    pub fn so_ea_one(&self) -> BitBoard {
        (*self >> 7) & constants::NOT_A_FILE
    }

    pub fn west_one(&self) -> BitBoard {
        (*self >> 1) & constants::NOT_H_FILE
    }

    pub fn so_we_one(&self) -> BitBoard {
        (*self >> 9) & constants::NOT_H_FILE
    }

    pub fn no_we_one(&self) -> BitBoard {
        (*self << 7) & constants::NOT_H_FILE
    }

    pub fn rotate_left(&self, steps: usize) -> BitBoard {
        (*self << steps) | (*self >> (64 - steps))
    }

    pub fn rotate_right(&self, steps: usize) -> BitBoard {
        (*self >> steps) | (*self << (64 - steps))
    }

    /// Shifts left for positive amounts but right for negative ones
    pub fn gen_shift(&self, steps: i64) -> BitBoard {
        if steps > 0 {
            *self << (steps as usize)
        } else {
            *self >> (-steps as usize)
        }
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: BitBoard) -> Self::Output {
        BitBoard::from(self.0 & rhs.0)
    }
}

impl BitAnd<u64> for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: u64) -> Self::Output {
        BitBoard::from(self.0 & rhs)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: BitBoard) -> Self::Output {
        BitBoard::from(self.0 | rhs.0)
    }
}

impl BitOr<u64> for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: u64) -> Self::Output {
        BitBoard::from(self.0 | rhs)
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: BitBoard) -> Self::Output {
        BitBoard::from(self.0 ^ rhs.0)
    }
}

impl BitXor<u64> for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: u64) -> Self::Output {
        BitBoard::from(self.0 ^ rhs)
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard::from(!self.0)
    }
}

impl Shl<usize> for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: usize) -> Self::Output {
        BitBoard::from(self.0 << rhs)
    }
}

impl Shr<usize> for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: usize) -> Self::Output {
        BitBoard::from(self.0 >> rhs)
    }
}