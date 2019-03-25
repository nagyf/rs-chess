//! This module implements the `King`'s movement logic.

use crate::engine::board::bitboard::BitBoard;

#[cfg(test)]
mod tests;

/// Returns the attack targets of the kings.
///
/// # Example
///
/// ```
/// println!("{}", attack_targets(Square::from_pos(Rank::D, File::Fourth).as_bb()));
///
/// . . . . . . . .
/// . . . . . . . .
/// . . . . . . . .
/// . . x x x . . .
/// . . x K x . . .
/// . . x x x . . .
/// . . . . . . . .
/// . . . . . . . .
/// ```
pub fn attack_targets(kings: BitBoard) -> BitBoard {
    return kings.east_one()
        | kings.west_one()
        | kings.north_one()
        | kings.south_one()
        | kings.no_ea_one()
        | kings.no_we_one()
        | kings.so_ea_one()
        | kings.so_we_one();
}
