use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::square::constants::{NOT_A_FILE, NOT_AB_FILE, NOT_GH_FILE, NOT_H_FILE};

pub fn attack_targets(knights: BitBoard) -> BitBoard {
    let l1 = (knights >> 1) & NOT_H_FILE;
    let l2 = (knights >> 2) & NOT_GH_FILE;
    let r1 = (knights << 1) & NOT_A_FILE;
    let r2 = (knights << 2) & NOT_AB_FILE;
    let h1 = l1 | r1;
    let h2 = l2 | r2;
    (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8)
}
