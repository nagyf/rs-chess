use crate::engine::board::piece::color::Color;
use crate::engine::board::square::Square;
use crate::engine::board::piece::castling::CastlingRight;
use crate::engine::board::piece::{color, Piece};
use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::{piece, Board};

pub struct BoardBuilder {
    turn: Color,
    half_moves: u16,
    full_moves: u16,
    en_passant: Option<Square>,
    castling_rights: [CastlingRight; color::NUM_COLORS],
    colors: [BitBoard; color::NUM_COLORS],
    pieces: [BitBoard; piece::NUM_PIECES],
}

impl BoardBuilder {
    pub fn new() -> BoardBuilder {
        BoardBuilder {
            turn: Color::White,
            half_moves: 0,
            full_moves: 0,
            en_passant: None,
            castling_rights: [CastlingRight::NoRight, CastlingRight::NoRight],
            colors: [BitBoard::new(); color::NUM_COLORS],
            pieces: [BitBoard::new(); piece::NUM_PIECES],
        }
    }

    pub fn set_turn(&mut self, color: Color) -> &mut BoardBuilder {
        self.turn = color;
        self
    }

    pub fn set_half_moves(&mut self, half_moves: u16) -> &mut BoardBuilder {
        self.half_moves = half_moves;
        self
    }

    pub fn set_full_moves(&mut self, full_moves: u16) -> &mut BoardBuilder {
        self.full_moves = full_moves;
        self
    }

    pub fn set_en_passant(&mut self, en_passant: Option<Square>) -> &mut BoardBuilder {
        self.en_passant = en_passant;
        self
    }

    pub fn set_castling_right(&mut self, color: Color, castling_right: CastlingRight) -> &mut BoardBuilder {
        self.castling_rights[color.to_index()] = castling_right;
        self
    }

    pub fn set_castling_rights(&mut self, castling_rights: [CastlingRight; 2]) -> &mut BoardBuilder {
        self.castling_rights = castling_rights;
        self
    }

    pub fn set_color(&mut self, color: Color, bitboard: BitBoard) -> &mut BoardBuilder {
        self.colors[color.to_index()] = bitboard;
        self
    }

    pub fn add_piece(&mut self, piece_type: Piece, color: Color, square: Square) -> &mut BoardBuilder {
        self.pieces[piece_type.to_index()] = self.pieces[piece_type.to_index()].set(square);
        self.colors[color.to_index()] = self.colors[color.to_index()].set(square);
        self
    }

    pub fn build(&self) -> Board {
        let mut board = Board::empty();
        board.turn = self.turn;
        board.half_moves = self.half_moves;
        board.full_moves = self.full_moves;
        board.en_passant = self.en_passant;
        board.castling_rights = self.castling_rights;
        board.colors = self.colors;
        board.pieces = self.pieces;
        board
    }
}
