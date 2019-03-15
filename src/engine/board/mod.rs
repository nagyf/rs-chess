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
    pub turn: piece::Color,
    pub half_moves: u16,
    pub full_moves: u16,
    pub en_passant: Option<Square>,
    pub castling_rights: [CastlingRight; piece::NUM_COLORS],
    white: BitBoard,
    black: BitBoard,
    pieces: [BitBoard; piece::NUM_PIECES],
}

impl Board {
    /// Constructs a completely empty board.
    ///
    /// If you want an initial board instead, use Board::new() or default()
    pub fn empty() -> Board {
        let mut pieces = [BitBoard::new(); piece::NUM_PIECES];
        for piece in &piece::ALL_PIECES {
            pieces[piece.to_index()] = BitBoard::new();
        }

        Board {
            turn: piece::Color::White,
            half_moves: 0,
            full_moves: 0,
            en_passant: None,
            castling_rights: [NoRight, NoRight],
            white: BitBoard::from(0x0000000000000000),
            black: BitBoard::from(0x0000000000000000),
            pieces,
        }
    }

    /// Constructs a new board with every piece in it's initial position.
    ///
    /// If you want a completely empty board, use Board::empty() instead.
    pub fn new() -> Board {
        let mut pieces= [BitBoard::new(); piece::NUM_PIECES];
        for piece in &piece::ALL_PIECES {
            pieces[piece.to_index()] = Board::initial_position(*piece);
        }

        Board {
            turn: piece::Color::White,
            half_moves: 0,
            full_moves: 0,
            en_passant: None,
            castling_rights: [BothSide, BothSide],
            white: BitBoard::from(0x000000000000FFFF),
            black: BitBoard::from(0xFFFF000000000000),
            pieces,
        }
    }

    fn initial_position(piece: Piece) -> BitBoard {
        match piece {
            Piece::Pawn => Board::pawns(),
            Piece::Rook => Board::rooks(),
            Piece::Knight => Board::knights(),
            Piece::Bishop => Board::bishops(),
            Piece::King => Board::kings(),
            Piece::Queen => Board::queens(),
        }
    }

    /// Returns the bitboard representing the starting positions of pawns
    fn pawns() -> BitBoard {
        BitBoard::from(0x00FF00000000FF00)
    }

    /// Returns the bitboard representing the starting positions of rooks
    fn rooks() -> BitBoard {
        BitBoard::from(0x8100000000000081)
    }

    /// Returns the bitboard representing the starting positions of knights
    fn knights() -> BitBoard {
        BitBoard::from(0x4200000000000042)
    }

    /// Returns the bitboard representing the starting positions of bishops
    fn bishops() -> BitBoard {
        BitBoard::from(0x2400000000000024)
    }

    /// Returns the bitboard representing the starting positions of kings
    fn kings() -> BitBoard {
        BitBoard::from(0x1000000000000010)
    }

    /// Returns the bitboard representing the starting positions of queens
    fn queens() -> BitBoard {
        BitBoard::from(0x0800000000000008)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let board = self.white | self.black;

        write!(f, "{}", board)
    }
}

impl Board {
    pub fn add_piece_mut(&mut self, piece_type: piece::Piece, color: piece::Color, rank: Rank, file: File) -> &Board {
        self.pieces[piece_type.to_index()] = self.pieces[piece_type.to_index()].set(rank, file);

        match color {
            Color::Black => {
                self.black = self.black.set(rank, file);
            }
            Color::White => {
                self.white = self.white.set(rank, file);
            }
        }

        self
    }
}