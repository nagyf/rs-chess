use std::fmt::{Display, Error, Formatter};

use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::piece::{CastlingRight, Color, Piece};
use crate::engine::board::piece::CastlingRight::{BothSide, NoRight};
use crate::engine::board::square::{File, Rank, Square};

mod constants;
pub mod bitboard;
pub mod piece;
pub mod square;
pub mod chessmove;

#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Board {
    pub turn: Color,
    pub half_moves: u16,
    pub full_moves: u16,
    pub en_passant: Option<Square>,
    pub castling_rights: [CastlingRight; piece::NUM_COLORS],
    pub colors: [BitBoard; piece::NUM_COLORS],
    pieces: [BitBoard; piece::NUM_PIECES],
}

impl Board {
    /// Constructs a completely empty board.
    ///
    /// If you want an initial board instead, use Board::new() or default()
    pub fn empty() -> Board {
        Board {
            turn: Color::White,
            half_moves: 0,
            full_moves: 0,
            en_passant: None,
            castling_rights: [NoRight, NoRight],
            colors: [BitBoard::new(); piece::NUM_COLORS],
            pieces: [BitBoard::new(); piece::NUM_PIECES],
        }
    }

    /// Constructs a new board with every piece in it's initial position.
    ///
    /// If you want a completely empty board, use Board::empty() instead.
    pub fn new() -> Board {
        let mut pieces = [BitBoard::new(); piece::NUM_PIECES];
        for piece in &piece::ALL_PIECES {
            pieces[piece.to_index()] = piece.initial_position();
        }

        let mut colors = [BitBoard::new(); piece::NUM_COLORS];
        colors[Color::White.to_index()] = Color::White.initial_position();
        colors[Color::Black.to_index()] = Color::Black.initial_position();

        Board {
            turn: Color::White,
            half_moves: 0,
            full_moves: 0,
            en_passant: None,
            castling_rights: [BothSide, BothSide],
            colors,
            pieces,
        }
    }

    pub fn get_pieces(&self, piece: Piece) -> BitBoard {
        self.pieces[piece.to_index()]
    }

    pub fn get_pieces_color(&self, piece: Piece) -> BitBoard {
        self.pieces[piece.to_index()]
    }

    pub fn add_piece(&mut self, piece_type: Piece, color: Color, rank: Rank, file: File) -> Board {
        let mut result = *self;
        result.pieces[piece_type.to_index()] = self.pieces[piece_type.to_index()].set(rank, file);
        result.colors[color.to_index()] = result.colors[color.to_index()].set(rank, file);
        result
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut board = BitBoard::new();
        for color in &piece::ALL_COLORS {
            board = board | self.colors[color.to_index()];
        }


        write!(f, "{}", board)
    }
}
