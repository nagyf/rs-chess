// TODO: remove this
#![allow(unused)]

use crate::engine::board::bitboard::BitBoard;

pub static LIGHT_SQUARES: BitBoard = BitBoard(0x55AA55AA55AA55AA);
pub static DARK_SQUARES: BitBoard = BitBoard(0xAA55AA55AA55AA55);

pub static DIAGONAL_A1_H8: BitBoard = BitBoard(0x8040201008040201);
pub static ANTIDIAGONAL_H1_A8: BitBoard = BitBoard(0x0102040810204080);

pub static EMPTY: BitBoard = BitBoard(0x00000000000000FF);
pub static UNIVERSE: BitBoard = BitBoard(0xFFFFFFFFFFFFFFFF);