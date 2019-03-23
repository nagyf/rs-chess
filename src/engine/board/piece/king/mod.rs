use crate::engine::board::bitboard::BitBoard;

#[cfg(test)]
mod tests;

/// Returns the attack targets of the kings.
pub fn attack_targets(kings: BitBoard) -> BitBoard {
    let attacks = kings.east_one()
        | kings.west_one()
        | kings.north_one()
        | kings.south_one()
        | kings.no_ea_one()
        | kings.no_we_one()
        | kings.so_ea_one()
        | kings.so_we_one();
    attacks
}
