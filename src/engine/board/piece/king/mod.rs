use crate::engine::board::bitboard::BitBoard;

pub fn attack_targets(kings: BitBoard) -> BitBoard {
    let attacks = kings.east_one() | kings.west_one();
    let kings = kings | attacks;
    attacks | kings.north_one() | kings.south_one()
}
