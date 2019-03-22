use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::piece::Piece;
use crate::engine::board::square::constants::{FILE_A};
use crate::engine::board::square::Square;

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
    FILE_A << ((square.to_index() & 7) as usize)
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

fn rank_attacks(square: Square, occupied: BitBoard) -> BitBoard {
    let rank = ((square.get_rank().to_index() - 1) * 8) as usize;
    let mut occ = occupied >> rank;
    let mut pc = square.as_bb() >> rank;
    occ.0 = occ.0.wrapping_mul(FILE_A.0);
    pc.0 = pc.0.wrapping_mul(FILE_A.0);
    occ &= 0x8040201008040201;
    pc &= 0x8040201008040201;
    let diag_attacks = diagonal_attacks(Square::from_bb(pc), occ);
    let result = (diag_attacks.0.wrapping_mul(FILE_A.0) / 0x0100000000000000) << rank;
    BitBoard(result)
}