use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::square::{File, Rank, Square};
use crate::engine::board::piece::{Color, CastlingRight};

mod constants;
pub mod bitboard;
pub mod piece;
pub mod square;
pub mod chessmove;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Board {
    pub turn: piece::Color,
    pub half_moves: u16,
    pub full_moves: u16,
    pub en_passant: Option<Square>,
    pub castling_rights: Vec<CastlingRight>,
    white: BitBoard,
    black: BitBoard,
    pieces: HashMap<piece::Piece, BitBoard>,
}

impl Board {

    /// Constructs a completely empty board.
    ///
    /// If you want an initial board instead, use Board::new() or default()
    pub fn empty() -> Board {
        let mut pieces = HashMap::new();
        pieces.insert(piece::Piece::Pawn, BitBoard::empty());
        pieces.insert(piece::Piece::Rook, BitBoard::empty());
        pieces.insert(piece::Piece::Knight, BitBoard::empty());
        pieces.insert(piece::Piece::Bishop, BitBoard::empty());
        pieces.insert(piece::Piece::King, BitBoard::empty());
        pieces.insert(piece::Piece::Queen, BitBoard::empty());

        Board {
            turn: piece::Color::White,
            half_moves: 0,
            full_moves: 0,
            en_passant: None,
            castling_rights: Vec::new(),
            white: BitBoard::from(0x0000000000000000),
            black: BitBoard::from(0x0000000000000000),
            pieces,
        }
    }

    /// Constructs a new board with every piece in it's initial position.
    ///
    /// If you want a completely empty board, use Board::empty() instead.
    pub fn new() -> Board {
        let mut pieces = HashMap::new();
        pieces.insert(piece::Piece::Pawn, Board::pawns());
        pieces.insert(piece::Piece::Rook, Board::rooks());
        pieces.insert(piece::Piece::Knight, Board::knights());
        pieces.insert(piece::Piece::Bishop, Board::bishops());
        pieces.insert(piece::Piece::King, Board::kings());
        pieces.insert(piece::Piece::Queen, Board::queens());

        Board {
            turn: piece::Color::White,
            half_moves: 0,
            full_moves: 0,
            en_passant: None,
            castling_rights: Vec::new(),
            white: BitBoard::from(0x000000000000FFFF),
            black: BitBoard::from(0xFFFF000000000000),
            pieces,
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
        match self.pieces.get(&piece_type) {
            None => {
                self.pieces.insert(piece_type, BitBoard::empty().set(rank, file));
            }
            Some(bb) => {
                self.pieces.insert(piece_type, bb.set(rank, file));
            }
        };

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