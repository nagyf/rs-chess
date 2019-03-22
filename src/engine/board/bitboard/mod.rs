use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign};

use crate::engine::board::constants::{DEBRUJN_64, EMPTY, INDEX_64, UNIVERSE};
use crate::engine::board::square::{constants, File, Rank, Square};

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

impl BitBoard {
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
    /// Creates an empty bitboard
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a bitboard from a u64 value
    pub fn from(value: u64) -> Self {
        BitBoard(value)
    }

    pub fn from_square(square: Square) -> Self {
        BitBoard::from(1u64 << square.to_index())
    }

    /// Creates an empty bitboard
    pub fn empty() -> Self {
        Self::new()
    }

    /// Creates a bitboard in which every bit is set to 1
    pub fn universe() -> Self {
        UNIVERSE
    }
}

impl BitBoard {
    /// Returns the underlying value of the board as a u64 number
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Checks whether the board is empty
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Takes the board's complement to determine the empty fields
    pub fn empty_squares(&self) -> BitBoard {
        !(*self)
    }

    /// Negate every bit in the board, so ones become zeros, and zeros become ones
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

    /// Determines how many bits are set to 1 (i.e. how many pieces are on the board)
    pub fn pop_count(&self) -> u64 {
        let mut board = self.0;
        let mut count = 0;
        while board != 0 {
            count += 1;
            board &= board - 1;
        }

        count
    }

    /// Flips the board vertically about the center ranks.
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

    /// Mirrors the board horizontally about the center files.
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
    /// Square a1 is mapped to h8 and vice versa.
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

    /// Rotates the board by 180 degrees
    pub fn rotate_180(&self) -> BitBoard {
        self.mirror_horizontal().flip_vertical()
    }

    /// Rotate the board 90 degrees clockwise
    pub fn rotate_90_cw(&self) -> BitBoard {
        self.flip_diag_a1_h8().flip_vertical()
    }

    /// Rotate the board 90 degrees counter-clockwise
    pub fn rotate_90_ccw(&self) -> BitBoard {
        self.flip_vertical().flip_diag_a1_h8()
    }

    pub fn is_set(&self, rank: Rank, file: File) -> bool {
        let square_index = BitBoard::square_index(rank, file);
        let x: u64 = 1 << square_index;
        (*self & x).value() != 0
    }

    pub fn set(&self, rank: Rank, file: File) -> BitBoard {
        let square_index = BitBoard::square_index(rank, file);
        let x: u64 = 1 << square_index;
        *self | x
    }

    pub fn toggle(&self, rank: Rank, file: File) -> BitBoard {
        let square_index = BitBoard::square_index(rank, file);
        let x: u64 = 1 << square_index;
        *self ^ x
    }

    pub fn bit_scan_fw(&self) -> u64 {
        assert_ne!(*self, EMPTY);
        INDEX_64[(((self.0 ^ (self.0 - 1)).wrapping_mul(DEBRUJN_64.0)) >> 58) as usize]
    }

    fn square_index(rank: Rank, file: File) -> u64 {
        let r = (rank.to_index() - 1) as u64;
        let f = (file.to_index() - 1) as u64;
        8 * r + f
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