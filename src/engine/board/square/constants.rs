// TODO: remove this
#![allow(unused)]

use crate::engine::board::bitboard::BitBoard;

pub const RANK_1: BitBoard = BitBoard(0x00000000000000FF);
pub const RANK_2: BitBoard = BitBoard(0x000000000000FF00);
pub const RANK_3: BitBoard = BitBoard(0x0000000000FF0000);
pub const RANK_4: BitBoard = BitBoard(0x00000000FF000000);
pub const RANK_5: BitBoard = BitBoard(0x000000FF00000000);
pub const RANK_6: BitBoard = BitBoard(0x0000FF0000000000);
pub const RANK_7: BitBoard = BitBoard(0x00FF000000000000);
pub const RANK_8: BitBoard = BitBoard(0xFF00000000000000);

pub const FILE_A: BitBoard = BitBoard(0x0101010101010101);
pub const FILE_B: BitBoard = BitBoard(0x0202020202020202);
pub const FILE_C: BitBoard = BitBoard(0x0404040404040404);
pub const FILE_D: BitBoard = BitBoard(0x0808080808080808);
pub const FILE_E: BitBoard = BitBoard(0x1010101010101010);
pub const FILE_F: BitBoard = BitBoard(0x2020202020202020);
pub const FILE_G: BitBoard = BitBoard(0x4040404040404040);
pub const FILE_H: BitBoard = BitBoard(0x8080808080808080);

pub const NOT_A_FILE: BitBoard = BitBoard(0xFEFEFEFEFEFEFEFE);
pub const NOT_H_FILE: BitBoard = BitBoard(0x7F7F7F7F7F7F7F7F);
pub const NOT_AB_FILE: BitBoard = BitBoard(0xFCFCFCFCFCFCFCFC);
pub const NOT_GH_FILE: BitBoard = BitBoard(0x3F3F3F3F3F3F3F3F);