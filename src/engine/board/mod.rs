use std::fmt::{Display, Error, Formatter};

use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::chessmove::ChessMove;
use crate::engine::board::constants::EMPTY;
use crate::engine::board::piece::{ALL_PIECES, color, general, king, knight, pawn, Piece, sliding};
use crate::engine::board::piece::castling::CastlingRight;
use crate::engine::board::piece::color::Color;
use crate::engine::board::square::Square;

mod constants;
pub mod bitboard;
pub mod piece;
pub mod square;
pub mod chessmove;
pub mod builder;
#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Board {
    turn: Color,
    half_moves: u16,
    full_moves: u16,
    en_passant: Option<Square>,
    castling_rights: [CastlingRight; color::NUM_COLORS],
    colors: [BitBoard; color::NUM_COLORS],
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
            castling_rights: [CastlingRight::NoRight, CastlingRight::NoRight],
            colors: [BitBoard::new(); color::NUM_COLORS],
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

        let mut colors = [BitBoard::new(); color::NUM_COLORS];
        colors[Color::White.to_index()] = Color::White.initial_position();
        colors[Color::Black.to_index()] = Color::Black.initial_position();

        Board {
            turn: Color::White,
            half_moves: 0,
            full_moves: 1,
            en_passant: None,
            castling_rights: [CastlingRight::BothSide, CastlingRight::BothSide],
            colors,
            pieces,
        }
    }

    /// Returns the color of the player who have to move .
    pub fn get_turn(&self) -> Color {
        return self.turn;
    }

    /// Returns the number of half moves.
    pub fn get_half_moves(&self) -> u16 {
        return self.half_moves;
    }

    /// Returns the number of full moves.
    pub fn get_full_moves(&self) -> u16 {
        return self.full_moves;
    }

    /// Returns en_passant target square, or `None`.
    pub fn get_en_passant(&self) -> Option<Square> {
        return self.en_passant;
    }

    /// Returns the castling rights.
    pub fn get_castling_rights(&self) -> [CastlingRight; 2] {
        return self.castling_rights;
    }

    /// Returns piece positions by piece type.
    pub fn get_pieces(&self, piece: Piece) -> BitBoard {
        self.pieces[piece.to_index()]
    }

    /// Returns piece positions by piece type and color.
    pub fn get_pieces_color(&self, piece: Piece, color: Color) -> BitBoard {
        self.pieces[piece.to_index()] & self.colors[color.to_index()]
    }

    /// Validates the move and makes it, if it is valid.
    ///
    /// Returns the new board after the move, or `None` if the move was invalid.
    pub fn make_move(&self, chess_move: ChessMove) -> Option<Board> {
        // Check if source is not empty
        if !self.is_valid_move(chess_move) {
            return None;
        }

        let piece = self.piece_at(chess_move.get_source(), self.turn).unwrap();

        // Check if the move is legal
        if !self.is_legal_move(piece, chess_move) {
            return None;
        }

        // Make sure the king doesn't remain in check at the end of the turn
        let result = self.make_move_without_validation(chess_move);

        if result.in_check(self.turn) {
            None
        } else {
            Some(result)
        }
    }

    /// Makes a move without any sanity checking or validity checking
    fn make_move_without_validation(&self, chess_move: ChessMove) -> Board {
        let piece = self.piece_at(chess_move.get_source(), self.turn).unwrap();
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
        return result;
    }

    /// This function checks whether the king of the specified color is in check.
    ///
    /// Returns `true` if it is check, otherwise returns `false`
    fn in_check(&self, color: Color) -> bool {
        let king = self.get_pieces_color(Piece::King, color);
        let king_in_check = general::square_attacked_by(Square::from_bb(king), self) != EMPTY;
        return king_in_check;
    }

    /// Returns the position of every piece on the board.
    pub fn pieces(&self) -> BitBoard {
        self.colors[Color::White.to_index()] | self.colors[Color::Black.to_index()]
    }

    /// Returns a bitboard which contains only the squares that are empty.
    pub fn empty_squares(&self) -> BitBoard {
        !self.pieces()
    }

    /// Returns every piece by the specified color.
    pub fn pieces_by_color(&self, color: Color) -> BitBoard {
        self.colors[color.to_index()]
    }

    /// Returns the current player's pieces.
    pub fn own_pieces(&self) -> BitBoard {
        self.pieces_by_color(self.turn)
    }

    /// Returns the enemy player's pieces.
    pub fn enemy_pieces(&self) -> BitBoard {
        self.pieces_by_color(!self.turn)
    }

    /// Returns every piece by the specified type.
    pub fn pieces_by_type(&self, piece: Piece) -> BitBoard {
        self.pieces[piece.to_index()]
    }

    /// Sets the specified piece and color in the specified positions.
    ///
    /// Mutates the board.
    fn xor(&mut self, piece: Piece, color: Color, bb: BitBoard) {
        self.colors[color.to_index()] ^= bb;
        self.pieces[piece.to_index()] ^= bb;
    }

    /// This function runs a sanity check before piece-wise move validation.
    ///
    /// Checks basic things, like if there is a piece at the source square, and it is our own piece.
    fn is_valid_move(&self, chess_move: ChessMove) -> bool {
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

    /// Piece-wise move validation
    fn is_legal_move(&self, piece: Piece, chess_move: ChessMove) -> bool {
        match piece {
            Piece::Pawn => {
                let valid_moves = pawn::push_targets(self.turn,
                                                     chess_move.get_source().as_bb(),
                                                     self.empty_squares());
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
                let attack_targets = king::attack_targets(chess_move.get_source().as_bb());
                let valid_moves = attack_targets & !self.own_pieces();
                // Check if there are anyone attacking the destination
                let dst_attackers = general::square_attacked_by(
                    chess_move.get_destination(),
                    self) & self.enemy_pieces();

                (valid_moves & chess_move.get_destination().as_bb() != EMPTY) && dst_attackers == EMPTY
            }
            _ => {
                let attack_targets = sliding::get_piece_attacks(piece, chess_move.get_source(), self.pieces());
                (attack_targets ^ self.own_pieces()) & chess_move.get_destination().as_bb() != EMPTY
            }
        }
    }

    /// Returns the piece at the specified square, from the specified color.
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
        for color in &color::ALL_COLORS {
            board = board | self.colors[color.to_index()];
        }


        write!(f, "{}", board)
    }
}
