//! General movement related logics implemented in this module.

use crate::engine::board::{Board, piece};
use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::piece::{color, king, knight, pawn, Piece, sliding};
use crate::engine::board::piece::color::Color;
use crate::engine::board::square::Square;

#[cfg(test)]
mod tests;

/// This function checks whether there are pieces attacking a particular square.
///
/// It will return a bitboard with every attackers in it.
pub fn square_attacked_by(square: Square, board: &Board) -> BitBoard {
    square_attacked_by_helper(square, board.pieces, board.colors, board.pieces())
}

fn square_attacked_by_helper(square: Square,
                          pieces: [BitBoard; piece::NUM_PIECES],
                          colors: [BitBoard; color::NUM_COLORS],
                          occupied: BitBoard) -> BitBoard {
    let black_pawns = pieces[Piece::Pawn.to_index()] & colors[Color::Black.to_index()];
    let white_pawns = pieces[Piece::Pawn.to_index()] & colors[Color::White.to_index()];
    let knights = pieces[Piece::Knight.to_index()];
    let sliding_pieces = pieces[Piece::Queen.to_index()]
        | pieces[Piece::Rook.to_index()]
        | pieces[Piece::Bishop.to_index()];
    let kings = pieces[Piece::King.to_index()];

    return (pawn::any_valid_attack(Color::White, square.as_bb(), occupied ^ white_pawns) & black_pawns)
        | (pawn::any_valid_attack(Color::Black, square.as_bb(), occupied ^ black_pawns) & white_pawns)
        | (knight::attack_targets(square.as_bb()) & knights)
        | (sliding::get_piece_attacks(Piece::Queen, square, occupied) & sliding_pieces)
        | (king::attack_targets(square.as_bb()) & kings);
}
