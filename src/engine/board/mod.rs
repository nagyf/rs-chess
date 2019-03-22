use std::fmt::{Display, Error, Formatter};

use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::chessmove::ChessMove;
use crate::engine::board::constants::EMPTY;
use crate::engine::board::piece::{ALL_PIECES, CastlingRight, Color, king, knight, pawn, Piece, sliding};
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
    turn: Color,
    half_moves: u16,
    full_moves: u16,
    en_passant: Option<Square>,
    castling_rights: [CastlingRight; piece::NUM_COLORS],
    colors: [BitBoard; piece::NUM_COLORS],
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
            full_moves: 1,
            en_passant: None,
            castling_rights: [BothSide, BothSide],
            colors,
            pieces,
        }
    }

    pub fn get_turn(&self) -> Color {
        return self.turn;
    }

    pub fn get_half_moves(&self) -> u16 {
        return self.half_moves;
    }

    pub fn get_full_moves(&self) -> u16 {
        return self.full_moves;
    }

    pub fn get_en_passant(&self) -> Option<Square> {
        return self.en_passant;
    }

    pub fn get_castling_rights(&self) -> [CastlingRight; 2] {
        return self.castling_rights;
    }

    pub fn get_pieces(&self, piece: Piece) -> BitBoard {
        self.pieces[piece.to_index()]
    }

    pub fn get_pieces_color(&self, piece: Piece, color: Color) -> BitBoard {
        self.pieces[piece.to_index()] & self.colors[color.to_index()]
    }

    pub fn add_piece(&mut self, piece_type: Piece, color: Color, rank: Rank, file: File) -> Board {
        let mut result = *self;
        result.pieces[piece_type.to_index()] = self.pieces[piece_type.to_index()].set(rank, file);
        result.colors[color.to_index()] = result.colors[color.to_index()].set(rank, file);
        result
    }

    pub fn make_move(&self, chess_move: ChessMove) -> Option<Board> {
        // Check if source is not empty
        if !self.is_valid_move(&chess_move) {
            return None;
        }

        let piece = self.piece_at(chess_move.get_source(), self.turn).unwrap();

        // Check if the move is legal
        if !self.is_legal_move(piece, &chess_move) {
            return None;
        }

        let src = chess_move.get_source().as_bb();
        let dst = chess_move.get_destination().as_bb();

        // Create the result
        let mut result = *self;
        // Move the piece
        result.xor(piece, self.turn, src);
        result.xor(piece, self.turn, dst);

        // If there was a capture, remove that piece
        if let Some(captured) = self.piece_at(chess_move.get_destination(), !self.turn) {
            result.xor(captured, !self.turn, dst);
        }

        result.half_moves += 1;
        if result.turn == Color::Black {
            result.full_moves += 1;
        }
        result.turn = !result.turn;
        Some(result)
    }

    pub fn pieces(&self) -> BitBoard {
        self.colors[Color::White.to_index()] | self.colors[Color::Black.to_index()]
    }

    pub fn empty_bb(&self) -> BitBoard {
        !self.pieces()
    }

    pub fn pieces_by_color(&self, color: Color) -> BitBoard {
        self.colors[color.to_index()]
    }

    pub fn own_pieces(&self) -> BitBoard {
        self.pieces_by_color(self.turn)
    }

    pub fn enemy_pieces(&self) -> BitBoard {
        self.pieces_by_color(!self.turn)
    }

    pub fn pieces_by_type(&self, piece: Piece) -> BitBoard {
        self.pieces[piece.to_index()]
    }

    fn xor(&mut self, piece: Piece, color: Color, bb: BitBoard) {
        self.colors[color.to_index()] ^= bb;
        self.pieces[piece.to_index()] ^= bb;
    }

    fn is_valid_move(&self, chess_move: &ChessMove) -> bool {
        let src = chess_move.get_source().as_bb();
        let dst = chess_move.get_destination().as_bb();

        // Make sure we have a piece at the source square
        if self.own_pieces() & src == EMPTY {
            return false;
        }

        // Make sure that the target square is not a king (you cannot capture kings)
        if self.pieces_by_type(Piece::King) & dst != EMPTY {
            return false;
        }

        true
    }

    fn is_legal_move(&self, piece: Piece, chess_move: &ChessMove) -> bool {
        match piece {
            Piece::Pawn => {
                let valid_moves = pawn::push_targets(self.turn,
                                                     chess_move.get_source().as_bb(),
                                                     self.empty_bb());
                let valid_attacks = pawn::any_valid_attack(self.turn,
                                                          chess_move.get_source().as_bb(),
                                                          self.enemy_pieces());

                (valid_moves | valid_attacks) & chess_move.get_destination().as_bb() != EMPTY
            }
            Piece::Knight => {
                // Where the knight can move
                let attack_targets = knight::attack_targets(chess_move.get_source().as_bb());
                // Consider only the empty and the enemy occupied squares
                let valid_moves = attack_targets & !self.own_pieces();
                valid_moves & chess_move.get_destination().as_bb() != EMPTY
            }
            Piece::King => {
                // TODO consider non-check squares
                let attack_targets = king::attack_targets(chess_move.get_source().as_bb());
                let valid_moves = attack_targets & !self.own_pieces();
                valid_moves & chess_move.get_destination().as_bb() != EMPTY
            }
            _ => {
                let attack_targets = sliding::get_piece_attacks(piece, chess_move.get_source(), self.pieces());
                (attack_targets ^ self.own_pieces()) & chess_move.get_destination().as_bb() != EMPTY
            }
        }
    }

    pub fn piece_at(&self, square: Square, color: Color) -> Option<Piece> {
        let pos = square.as_bb();

        if self.pieces_by_color(color) & pos == EMPTY {
            None
        } else {
            for piece in &ALL_PIECES {
                if self.pieces[piece.to_index()] & pos != EMPTY {
                    return Some(*piece);
                }
            }

            None
        }
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

pub struct BoardBuilder {
    turn: Color,
    half_moves: u16,
    full_moves: u16,
    en_passant: Option<Square>,
    castling_rights: [CastlingRight; piece::NUM_COLORS],
    colors: [BitBoard; piece::NUM_COLORS],
    pieces: [BitBoard; piece::NUM_PIECES],
}

impl BoardBuilder {
    pub fn new() -> BoardBuilder {
        BoardBuilder {
            turn: Color::White,
            half_moves: 0,
            full_moves: 0,
            en_passant: None,
            castling_rights: [NoRight, NoRight],
            colors: [BitBoard::new(); piece::NUM_COLORS],
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

    pub fn add_piece(&mut self, piece_type: Piece, color: Color, rank: Rank, file: File) -> &mut BoardBuilder {
        self.pieces[piece_type.to_index()] = self.pieces[piece_type.to_index()].set(rank, file);
        self.colors[color.to_index()] = self.colors[color.to_index()].set(rank, file);
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