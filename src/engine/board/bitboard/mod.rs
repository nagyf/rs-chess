//! This module implements a [BitBoard](https://www.chessprogramming.org/Bitboards) as a chessboard representation.
//!
//! Implementation follows the `Little endian rank-file mapping`, so the bits in the bitboard follows the following enumeration:
//! ```
//! h | 57 58 59 60 61 62 63 64
//! g | 49 50 51 52 53 54 55 56
//! f | 41 42 43 44 45 46 47 48
//! e | 33 34 35 36 37 38 39 40
//! d | 25 26 27 28 29 30 31 32
//! c | 17 18 19 20 21 22 23 24
//! b |  9 10 11 12 13 14 15 16
//! a |  1  2  3  4  5  6  7  8
//!   |------------------------
//!      1  2  3  4  5  6  7  8
//! ```
//!
//! The bitboard is represented with a `64bit unsigned integer` value as follows:
//! ```
//! 0x0000000000000000
//!   ^^            ^^
//! top row     bottom row
//! ```
//!

use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign};

use crate::engine::board::constants::{DEBRUJN_64, EMPTY, INDEX_64, UNIVERSE};
use crate::engine::board::square::{constants, Square};

#[cfg(test)]
mod tests;

///
/// Bitboard implemented with Little endian rank-file (LERF) mapping.
///
/// For more information: [Little-Endian Rank-File Mapping](https://www.chessprogramming.org/Square_Mapping_Considerations#Little-Endian_Rank-File_Mapping)
///
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct BitBoard(pub u64);

impl Default for BitBoard {
    /// Returns the default BitBoard, which is empty, or `0`.
    fn default() -> Self {
        EMPTY
    }
}

impl BitBoard {
    /// Formats the BitBoard as a chessboard.
    ///
    /// Useful for debugging purposes.
    /// `occupied` will be used for bits of `1`, `empty` will be used for bits of `0`.
    ///
    /// # Example
    ///
    /// ```
    /// let bb = BitBoard::new(0x0101010101010101);
    /// println!("{}", bb.format_with("X ", ". "));
    ///
    /// X . . . . . . .
    /// X . . . . . . .
    /// X . . . . . . .
    /// X . . . . . . .
    /// X . . . . . . .
    /// X . . . . . . .
    /// X . . . . . . .
    /// X . . . . . . .
    /// ```
    pub fn format_with(&self, occupied: &str, empty: &str) -> String {
        let mut result: String = "".to_owned();
        let mut row: String = "".to_owned();
        for x in 0..64 {
            if self.0 & (1u64 << x) == (1u64 << x) {
                row.push_str(occupied);
            } else {
                row.push_str(empty);
            }
            if x % 8 == 7 {
                row.push_str("\n");
                result.insert_str(0, &row);
                row.clear();
            }
        }
        result
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let result = self.format_with("X ", ". ");
        write!(f, "{}", result)
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#018X}", self.0)
    }
}

impl BitBoard {
    /// Creates an empty bitboard, meaning its' value will be `0`.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(0, BitBoard::new().0);
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a bitboard from a `u64` value.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(0x0000000000000008, BitBoard::from(8).0);
    /// ```
    pub fn from(value: u64) -> Self {
        BitBoard(value)
    }

    /// Creates an empty bitboard, meaning its' value will be `0`.
    pub fn empty() -> Self {
        EMPTY
    }

    /// Creates a bitboard in which every bit is set to `1`.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(0xFFFFFFFFFFFFFFFF, BitBoard::universe().0);
    /// ```
    pub fn universe() -> Self {
        UNIVERSE
    }
}

impl BitBoard {
    /// Returns the underlying value of the bitboard as a `u64` number.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(42, BitBoard::from(42).value());
    /// ```
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Checks whether the bitboard is empty (equal to `0`).
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Checks whether the bitboard is *not* empty (greater than `0`).
    pub fn is_not_empty(&self) -> bool {
        self.0 != 0
    }

    /// Takes the bitboard's complement to determine the empty fields.
    ///
    /// Bits of `1`s becomes `0`s and vice-versa.
    pub fn empty_squares(&self) -> BitBoard {
        !(*self)
    }

    /// Negate every bit in the bitboard, so ones become zeros, and zeros become ones.
    pub fn complement(&self) -> BitBoard {
        !*self
    }

    /// Moves every bit in the bitboard *up* one row.
    pub fn north_one(&self) -> BitBoard {
        *self << 8
    }

    /// Moves every bit in the bitboard *down* one row.
    pub fn south_one(&self) -> BitBoard {
        *self >> 8
    }

    /// Moves every bit in the bitboard *right* one file.
    pub fn east_one(&self) -> BitBoard {
        (*self << 1) & constants::NOT_1_FILE
    }

    /// Moves every bit in the bitboard *up and right* one rank and file.
    pub fn no_ea_one(&self) -> BitBoard {
        (*self << 9) & constants::NOT_1_FILE
    }

    /// Moves every bit in the bitboard *down and right* one rank and file.
    pub fn so_ea_one(&self) -> BitBoard {
        (*self >> 7) & constants::NOT_1_FILE
    }

    /// Moves every bit in the bitboard *left* one file.
    pub fn west_one(&self) -> BitBoard {
        (*self >> 1) & constants::NOT_8_FILE
    }

    /// Moves every bit in the bitboard *down and left* one rank and file.
    pub fn so_we_one(&self) -> BitBoard {
        (*self >> 9) & constants::NOT_8_FILE
    }

    /// Moves every bit in the bitboard *up and left* one rank and file.
    pub fn no_we_one(&self) -> BitBoard {
        (*self << 7) & constants::NOT_8_FILE
    }

    /// Rotates the board 90 degrees *counter-clockwise*.
    pub fn rotate_left(&self, steps: usize) -> BitBoard {
        (*self << steps) | (*self >> (64 - steps))
    }

    /// Rotates the board 90 degrees *clockwise*.
    pub fn rotate_right(&self, steps: usize) -> BitBoard {
        (*self >> steps) | (*self << (64 - steps))
    }

    /// Generic shift: shifts left for positive amounts but right for negative ones.
    pub fn gen_shift(&self, steps: i64) -> BitBoard {
        if steps > 0 {
            *self << (steps as usize)
        } else {
            *self >> (-steps as usize)
        }
    }

    /// Determines how many bits are set to 1 (i.e. how many pieces are on the bitboard).
    pub fn pop_count(&self) -> u64 {
        let mut bitboard = self.0;
        let mut count = 0;
        while bitboard != 0 {
            count += 1;
            bitboard &= bitboard - 1;
        }

        count
    }

    /// Flips the bitboard vertically about the center ranks.
    ///
    /// Rank 1 is mapped to rank 8 and vice versa.
    pub fn flip_vertical(&self) -> BitBoard {
        let x = self.0;
        let new_value = (x << 56) |
            ((x << 40) & 0x00ff000000000000) |
            ((x << 24) & 0x0000ff0000000000) |
            ((x << 8) & 0x000000ff00000000) |
            ((x >> 8) & 0x00000000ff000000) |
            ((x >> 24) & 0x0000000000ff0000) |
            ((x >> 40) & 0x000000000000ff00) |
            (x >> 56);
        BitBoard::from(new_value)
    }

    /// Mirrors the bitboard horizontally about the center files.
    ///
    /// File "a" is mapped to file "h" and vice versa.
    pub fn mirror_horizontal(&self) -> BitBoard {
        let mut x = self.0;
        let k1 = 0x5555555555555555;
        let k2 = 0x3333333333333333;
        let k4 = 0x0f0f0f0f0f0f0f0f;
        x = ((x >> 1) & k1) + 2 * (x & k1);
        x = ((x >> 2) & k2) + 4 * (x & k2);
        x = ((x >> 4) & k4) + 16 * (x & k4);
        BitBoard::from(x)
    }

    /// Flip a bitboard about the diagonal a1-h8.
    ///
    /// Square h1 is mapped to a8 and vice versa.
    pub fn flip_diag_a1_h8(&self) -> BitBoard {
        let mut x = self.0;
        let mut t;
        let k1 = 0x5500550055005500;
        let k2 = 0x3333000033330000;
        let k4 = 0x0f0f0f0f00000000;
        t = k4 & (x ^ (x << 28));
        x ^= t ^ (t >> 28);
        t = k2 & (x ^ (x << 14));
        x ^= t ^ (t >> 14);
        t = k1 & (x ^ (x << 7));
        x ^= t ^ (t >> 7);
        BitBoard::from(x)
    }

    /// Flip a bitboard about the antidiagonal a8-h1.
    ///
    /// Square a1 is mapped to h8 and vice-versa.
    pub fn flip_diag_a8_h1(&self) -> BitBoard {
        let mut x = self.0;
        let mut t;
        let k1 = 0xaa00aa00aa00aa00;
        let k2 = 0xcccc0000cccc0000;
        let k4 = 0xf0f0f0f00f0f0f0f;
        t = x ^ (x << 36);
        x ^= k4 & (t ^ (x >> 36));
        t = k2 & (x ^ (x << 18));
        x ^= t ^ (t >> 18);
        t = k1 & (x ^ (x << 9));
        x ^= t ^ (t >> 9);
        BitBoard::from(x)
    }

    /// Rotates the bitboard by 180 degrees.
    pub fn rotate_180(&self) -> BitBoard {
        self.mirror_horizontal().flip_vertical()
    }

    /// Rotate the bitboard 90 degrees clockwise.
    pub fn rotate_90_cw(&self) -> BitBoard {
        self.flip_diag_a1_h8().flip_vertical()
    }

    /// Rotate the bitboard 90 degrees counter-clockwise.
    pub fn rotate_90_ccw(&self) -> BitBoard {
        self.flip_vertical().flip_diag_a1_h8()
    }

    /// Checks whether a specific bit is set in the bitboard.
    pub fn is_set(&self, square: Square) -> bool {
        (*self & square.as_bb()).value() != 0
    }

    /// Sets a specific bit in the bitboard.
    pub fn set(&self, square: Square) -> BitBoard {
        *self | square.as_bb()
    }

    /// Toggles a specific bit in the bitboard.
    ///
    /// If it was `1` it becomes `0` and vice-versa.
    pub fn toggle(&self, square: Square) -> BitBoard {
        *self ^ square.as_bb()
    }

    /// Finds the Least Significant Bit that is `1`.
    ///
    /// The bitboard *must not* be empty when using this function.
    pub fn bit_scan_fw(&self) -> u64 {
        assert_ne!(*self, EMPTY);
        INDEX_64[(((self.0 ^ (self.0 - 1)).wrapping_mul(DEBRUJN_64.0)) >> 58) as usize]
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

impl ShlAssign<usize> for BitBoard {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl Shr<usize> for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: usize) -> Self::Output {
        BitBoard::from(self.0 >> rhs)
    }
}

impl ShrAssign<usize> for BitBoard {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

impl BitOrAssign<BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: BitBoard) {
        self.0 |= rhs.0;
    }
}

impl BitOrAssign<u64> for BitBoard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl BitAndAssign<BitBoard> for BitBoard {
    fn bitand_assign(&mut self, rhs: BitBoard) {
        self.0 &= rhs.0;
    }
}

impl BitAndAssign<u64> for BitBoard {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 &= rhs;
    }
}

impl BitXorAssign<BitBoard> for BitBoard {
    fn bitxor_assign(&mut self, rhs: BitBoard) {
        self.0 ^= rhs.0;
    }
}

impl BitXorAssign<u64> for BitBoard {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.0 ^= rhs;
    }
}
