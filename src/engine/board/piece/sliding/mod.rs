//! This module is used to calculate attack targets for sliding pieces (queen, rook, bishop).
//!
//! The module uses the `Hyperbola Quintessence` method to calculate the targets without lookups.
//!
//! For more information: [https://www.chessprogramming.org/Hyperbola_Quintessence](https://www.chessprogramming.org/Hyperbola_Quintessence)

use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::piece::Piece;
use crate::engine::board::square::{constants, Square};

#[cfg(test)]
mod tests;

pub fn get_piece_attacks(piece: Piece, square: Square, occupied: BitBoard) -> BitBoard {
    match piece {
        Piece::Rook => rook_attacks(square, occupied),
        Piece::Bishop => bishop_attacks(square, occupied),
        Piece::Queen => queen_attacks(square, occupied),
        _ => panic!("Invalid piece")
    }
}

pub fn rook_attacks(square: Square, occupied: BitBoard) -> BitBoard {
    rank_attacks(square, occupied) | file_attacks(square, occupied)
}

pub fn queen_attacks(square: Square, occupied: BitBoard) -> BitBoard {
    rook_attacks(square, occupied) | bishop_attacks(square, occupied)
}

pub fn bishop_attacks(square: Square, occupied: BitBoard) -> BitBoard {
    diagonal_attacks(square, occupied) | anti_diagonal_attacks(square, occupied)
}

fn file_mask(square: Square) -> BitBoard {
    constants::FILE_1 << ((square.to_index() & 7) as usize)
}

fn diagonal_mask(square: Square) -> BitBoard {
    let square_idx = square.to_index() as i64;
    let maindia = BitBoard::from(0x8040201008040201);
    let diag = 8 * (square_idx & 7) - (square_idx & 56);
    let nort = (-diag & (diag >> 31)) as usize;
    let sout = (diag & (-diag >> 31)) as usize;
    (maindia >> sout) << nort
}

fn anti_diagonal_mask(square: Square) -> BitBoard {
    let square_idx = square.to_index() as i64;
    let maindia = BitBoard::from(0x0102040810204080);
    let diag = 56 - 8 * (square_idx & 7) - (square_idx & 56);
    let nort = (-diag & (diag >> 31)) as usize;
    let sout = (diag & (-diag >> 31)) as usize;
    (maindia >> sout) << nort
}

fn diagonal_attacks(square: Square, occupied: BitBoard) -> BitBoard {
    let mut forward = occupied & diagonal_mask(square);
    let mut reverse = forward.flip_vertical();
    forward.0 = forward.0.wrapping_sub(square.as_bb().0);
    reverse.0 = reverse.0.wrapping_sub(square.as_bb().flip_vertical().0);
    forward ^= reverse.flip_vertical();
    forward &= diagonal_mask(square);
    forward
}

fn anti_diagonal_attacks(square: Square, occupied: BitBoard) -> BitBoard {
    let mut forward = occupied & anti_diagonal_mask(square);
    let mut reverse = forward.flip_vertical();
    forward.0 = forward.0.wrapping_sub(square.as_bb().0);
    reverse.0 = reverse.0.wrapping_sub(square.as_bb().flip_vertical().0);
    forward ^= reverse.flip_vertical();
    forward &= anti_diagonal_mask(square);
    forward
}

fn file_attacks(square: Square, occupied: BitBoard) -> BitBoard {
    let mut forward = occupied & file_mask(square);
    let mut reverse = forward.flip_vertical();
    forward.0 = forward.0.wrapping_sub(square.as_bb().0);
    reverse.0 = reverse.0.wrapping_sub(square.as_bb().flip_vertical().0);
    forward ^= reverse.flip_vertical();
    forward &= file_mask(square);
    forward
}

/// Pre-calculated rank attack target lookup table
lazy_static! {
    // 2048 Bytes = 2KByte
    static ref FIRST_RANK_ATTACKS: [[u8; 256]; 8] = init_first_rank_attacks();
}

/// Initializes the lookup table for rank attacks.
///
/// The resulting array will contain every possible combination of occupancies for all the 8 files
/// in a single rank.
fn init_first_rank_attacks() -> [[u8; 256]; 8] {
    let mut result: [[u8; 256]; 8] = [[0; 256]; 8];

    for file in 0..8 {
        for occupancy in 0..256 {
            result[file][occupancy] = single_rank_attacks(1u8 << file, occupancy as u8);
        }
    }

    result
}

/// Calculates the attack targets for a single rank (thus u8 used instead of u64).
fn single_rank_attacks(file: u8, occ: u8) -> u8 {
    left_rank_attacks(file, occ) | right_rank_attacks(file, occ)
}

/// Calculates the attack targets from the left of the specified file, considering the occupied squares.
///
/// TODO I'm sure this can be improved
fn left_rank_attacks(file: u8, occ: u8) -> u8 {
    let mut result = 0;
    let mut next = file << 1;

    while next > 0x00 {
        result ^= next;

        if occ & next != 0x00 {
            break;
        }

        next = next << 1;
    }

    result
}

/// Calculates the attack targets from the right of the specified file, considering the occupied squares.
///
/// TODO I'm sure this can be improved
fn right_rank_attacks(file: u8, occ: u8) -> u8 {
    let mut result = 0;
    let mut next = file >> 1;

    while next > 0x00 {
        result ^= next;

        if occ & next != 0x00 {
            break;
        }

        next = next >> 1;
    }

    result
}

fn rank_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    let file = sq.get_file().to_index() - 1;
    let rankx8 = ((sq.get_rank().to_index() - 1) * 8) as usize; // rank * 8

    // After shifting we only care about the first rank
    let rank_occurences = (occ.0 >> rankx8) as u8;

    // Search for the attack targets in the lookup table
    let attacks = FIRST_RANK_ATTACKS[file as usize][rank_occurences as usize] as u64;
    BitBoard(attacks << rankx8 as usize)
}
