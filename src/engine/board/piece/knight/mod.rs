//! This module implements the `Knight`'s movement logic.

use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::square::constants::{NOT_1_FILE, NOT_12_FILE, NOT_78_FILE, NOT_8_FILE};

#[cfg(test)]
mod tests;

/// Returns the attack targets of the knights.
///
/// # Example
///
/// ```
/// println!("{}", attack_targets(Square::from_pos(Rank::D, File::Fourth).as_bb()));
///
/// . . . . . . . .
/// . . . . . . . .
/// . . x . x . . .
/// . x . . . x . .
/// . . . N . . . .
/// . x . . . x . .
/// . . x . x . . .
/// . . . . . . . .
/// ```
pub fn attack_targets(knights: BitBoard) -> BitBoard {
    let l1 = (knights >> 1) & NOT_8_FILE;
    let l2 = (knights >> 2) & NOT_78_FILE;
    let r1 = (knights << 1) & NOT_1_FILE;
    let r2 = (knights << 2) & NOT_12_FILE;
    let h1 = l1 | r1;
    let h2 = l2 | r2;
    (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
}
