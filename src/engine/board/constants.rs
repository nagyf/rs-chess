// TODO: remove this
#![allow(unused)]

use crate::engine::board::bitboard::BitBoard;


pub static LIGHT_SQUARES: BitBoard = BitBoard(0x55AA55AA55AA55AA);
pub static DARK_SQUARES: BitBoard = BitBoard(0xAA55AA55AA55AA55);

pub static DIAGONAL_A1_H8: BitBoard = BitBoard(0x8040201008040201);
pub static ANTIDIAGONAL_H1_A8: BitBoard = BitBoard(0x0102040810204080);

pub static EMPTY: BitBoard = BitBoard(0x0000000000000000);
pub static UNIVERSE: BitBoard = BitBoard(0xFFFFFFFFFFFFFFFF);

pub static DEBRUJN_64: BitBoard = BitBoard(0x03f79d71b4cb0a89);
pub static INDEX_64: [u64; 64] = [
    0,  47,  1, 56, 48, 27,  2, 60,
    57, 49, 41, 37, 28, 16,  3, 61,
    54, 58, 35, 52, 50, 42, 21, 44,
    38, 32, 29, 23, 17, 11,  4, 62,
    46, 55, 26, 59, 40, 36, 15, 53,
    34, 51, 20, 43, 31, 22, 10, 45,
    25, 39, 14, 33, 19, 30,  9, 24,
    13, 18,  8, 12,  7,  6,  5, 63
];