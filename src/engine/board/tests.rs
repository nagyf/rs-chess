use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::Board;
use crate::engine::board::chessmove::ChessMove;
use crate::engine::board::piece::castling::CastlingRight;
use crate::engine::board::piece::color::Color;
use crate::engine::board::piece::Piece;
use crate::engine::board::square::{File, Rank, Square};

//
// Empty board
//

#[test]
fn empty_moves() {
    let board = Board::empty();
    assert_eq!(0, board.half_moves);
    assert_eq!(0, board.full_moves);
}

#[test]
fn empty_colors() {
    let board = Board::empty();
    assert_eq!(2, board.colors.len());
    // Check colors
    assert_eq!(BitBoard::new(), board.colors[Color::White.to_index()]);
    assert_eq!(BitBoard::new(), board.colors[Color::Black.to_index()]);
}

#[test]
fn empty_pieces() {
    let board = Board::empty();
    // Check pieces
    assert_eq!(BitBoard::new(), board.pieces[Piece::Pawn.to_index()]);
    assert_eq!(BitBoard::new(), board.pieces[Piece::Rook.to_index()]);
    assert_eq!(BitBoard::new(), board.pieces[Piece::Knight.to_index()]);
    assert_eq!(BitBoard::new(), board.pieces[Piece::Bishop.to_index()]);
    assert_eq!(BitBoard::new(), board.pieces[Piece::King.to_index()]);
    assert_eq!(BitBoard::new(), board.pieces[Piece::Queen.to_index()]);
}

#[test]
fn empty_en_passant() {
    let board = Board::empty();
    assert_eq!(None, board.en_passant);
}

#[test]
fn empty_castling() {
    let board = Board::empty();
    assert_eq!([CastlingRight::NoRight, CastlingRight::NoRight], board.castling_rights);
}

#[test]
fn empty_turn() {
    let board = Board::empty();
    assert_eq!(Color::White, board.turn);
}

//
// Initial board
//

#[test]
fn initial_half_moves() {
    let board = Board::new();
    assert_eq!(0, board.half_moves);
}

#[test]
fn initial_full_moves() {
    let board = Board::new();
    assert_eq!(1, board.full_moves);
}

#[test]
fn initial_colors() {
    let board = Board::new();
    assert_eq!(2, board.colors.len());
    // Check colors
    assert_eq!(Color::White.initial_position(), board.colors[Color::White.to_index()]);
    assert_eq!(Color::Black.initial_position(), board.colors[Color::Black.to_index()]);
}

#[test]
fn initial_pieces() {
    let board = Board::new();
    // Check pieces
    assert_eq!(Piece::Pawn.initial_position(), board.pieces[Piece::Pawn.to_index()]);
    assert_eq!(Piece::Rook.initial_position(), board.pieces[Piece::Rook.to_index()]);
    assert_eq!(Piece::Knight.initial_position(), board.pieces[Piece::Knight.to_index()]);
    assert_eq!(Piece::Bishop.initial_position(), board.pieces[Piece::Bishop.to_index()]);
    assert_eq!(Piece::King.initial_position(), board.pieces[Piece::King.to_index()]);
    assert_eq!(Piece::Queen.initial_position(), board.pieces[Piece::Queen.to_index()]);
}

#[test]
fn initial_en_passant() {
    let board = Board::new();
    assert_eq!(None, board.en_passant);
}

#[test]
fn initial_castling() {
    let board = Board::new();
    assert_eq!([CastlingRight::BothSide, CastlingRight::BothSide], board.castling_rights);
}

#[test]
fn initial_turn() {
    let board = Board::new();
    assert_eq!(Color::White, board.turn);
}

//
// Other methods
//

#[test]
fn get_turn() {
    assert_eq!(Color::White, Board::new().get_turn());
}

#[test]
fn get_turn2() {
    let mut board = Board::new();
    board.turn = Color::Black;

    assert_eq!(Color::Black, board.get_turn());
}

#[test]
fn get_half_moves() {
    let mut board = Board::new();
    board.half_moves = 42;

    assert_eq!(42, board.get_half_moves());
}

#[test]
fn get_full_moves() {
    let mut board = Board::new();
    board.full_moves = 42;

    assert_eq!(42, board.get_full_moves());
}

#[test]
fn get_castling_rights() {
    let mut board = Board::new();
    let expected = [CastlingRight::BothSide, CastlingRight::NoRight];
    board.castling_rights = expected;

    assert_eq!(expected, board.get_castling_rights());
}


#[test]
fn get_en_passant() {
    let mut board = Board::new();
    let expected = Some(Square::from_pos(Rank::C, File::Fourth));
    board.en_passant = expected;

    assert_eq!(expected, board.get_en_passant());
}

#[test]
fn in_check_empty() {
    let mut board = Board::empty();
    board.xor(Piece::King, Color::White, Square::from_pos(Rank::A, File::Fourth).as_bb());
    assert_eq!(false, board.in_check(Color::White));
}

#[test]
fn in_check_own_queen() {
    let mut board = Board::empty();
    board.xor(Piece::King, Color::White, Square::from_pos(Rank::A, File::Fourth).as_bb());
    board.xor(Piece::Queen, Color::White, Square::from_pos(Rank::A, File::First).as_bb());
    assert_eq!(false, board.in_check(Color::White));
}

#[test]
fn in_check_enemy_queen() {
    let mut board = Board::empty();
    board.xor(Piece::King, Color::White, Square::from_pos(Rank::A, File::Fourth).as_bb());
    board.xor(Piece::Queen, Color::Black, Square::from_pos(Rank::A, File::First).as_bb());
    assert_eq!(true, board.in_check(Color::White));
}

#[test]
fn pieces_empty() {
    let board = Board::empty();
    assert_eq!(BitBoard::empty(), board.pieces());
}

#[test]
fn pieces_initial() {
    let board = Board::new();
    assert_eq!(BitBoard::from(0xFFFF00000000FFFF), board.pieces());
}

#[test]
fn empty_squares_empty() {
    let board = Board::empty();
    assert_eq!(BitBoard::from(0xFFFFFFFFFFFFFFFF), board.empty_squares());
}

#[test]
fn empty_squares_initial() {
    let board = Board::new();
    assert_eq!(BitBoard::from(0x0000FFFFFFFF0000), board.empty_squares());
}

#[test]
fn pieces_by_color_empty() {
    let board = Board::empty();
    assert_eq!(BitBoard::from(0x0000000000000000), board.pieces_by_color(Color::White));
    assert_eq!(BitBoard::from(0x0000000000000000), board.pieces_by_color(Color::Black));
}

#[test]
fn pieces_by_color() {
    let board = Board::new();
    assert_eq!(BitBoard::from(0x000000000000FFFF), board.pieces_by_color(Color::White));
    assert_eq!(BitBoard::from(0xFFFF000000000000), board.pieces_by_color(Color::Black));
}

#[test]
fn own_pieces() {
    let mut board = Board::new();
    board.turn = Color::White;
    assert_eq!(BitBoard::from(0x000000000000FFFF), board.own_pieces());

    board.turn = Color::Black;
    assert_eq!(BitBoard::from(0xFFFF000000000000), board.own_pieces());
}

#[test]
fn enemy_pieces() {
    let mut board = Board::new();
    board.turn = Color::White;
    assert_eq!(BitBoard::from(0xFFFF000000000000), board.enemy_pieces());

    board.turn = Color::Black;
    assert_eq!(BitBoard::from(0x000000000000FFFF), board.enemy_pieces());
}

#[test]
fn pieces_by_type() {
    let board = Board::new();
    assert_eq!(Piece::Pawn.initial_position(), board.pieces_by_type(Piece::Pawn));
    assert_eq!(Piece::Rook.initial_position(), board.pieces_by_type(Piece::Rook));
    assert_eq!(Piece::Knight.initial_position(), board.pieces_by_type(Piece::Knight));
    assert_eq!(Piece::Bishop.initial_position(), board.pieces_by_type(Piece::Bishop));
    assert_eq!(Piece::King.initial_position(), board.pieces_by_type(Piece::King));
    assert_eq!(Piece::Queen.initial_position(), board.pieces_by_type(Piece::Queen));
}

#[test]
fn xor() {
    let mut board = Board::empty();
    board.xor(Piece::Pawn, Color::White, BitBoard::from(0x000000000000FF00));
    assert_eq!(Piece::Pawn.initial_position() & Color::White.initial_position(), board.pieces_by_type(Piece::Pawn));
}

#[test]
fn is_valid_move_src_dst() {
    // Source and destination is the same
    let board = Board::new();
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::A, File::First);
    assert_eq!(false, board.is_valid_move(ChessMove::new(src, dst)));
}

#[test]
fn is_valid_move_src_no_piece() {
    // There is no piece on the source square
    let board = Board::new();
    let src = Square::from_pos(Rank::C, File::First);
    let dst = Square::from_pos(Rank::D, File::First);
    assert_eq!(false, board.is_valid_move(ChessMove::new(src, dst)));
}

#[test]
fn is_valid_move_dst_own_piece() {
    // Tries to capture own piece
    let board = Board::new();
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::B, File::First);
    assert_eq!(false, board.is_valid_move(ChessMove::new(src, dst)));
}

#[test]
fn is_valid_move_dst_king() {
    // Tries to capture king
    let mut board = Board::empty();
    board.xor(Piece::King, Color::Black,
              Square::from_pos(Rank::H, File::First).as_bb());
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::H, File::First);
    assert_eq!(false, board.is_valid_move(ChessMove::new(src, dst)));
}

#[test]
fn is_valid_move_valid() {
    // Valid move
    let board = Board::new();
    let src = Square::from_pos(Rank::B, File::First);
    let dst = Square::from_pos(Rank::D, File::First);
    assert_eq!(true, board.is_valid_move(ChessMove::new(src, dst)));
}

#[test]
fn piece_at_empty() {
    // No piece there
    let board = Board::new();
    let square = Square::from_pos(Rank::C, File::First);
    assert_eq!(None, board.piece_at(square, Color::White));
}

#[test]
fn piece_at_enemy() {
    // Enemy piece there
    let board = Board::new();
    let square = Square::from_pos(Rank::H, File::First);
    assert_eq!(None, board.piece_at(square, Color::White));
}

#[test]
fn piece_at() {
    // Enemy piece there
    let board = Board::new();
    let square = Square::from_pos(Rank::B, File::First);
    assert_eq!(Some(Piece::Pawn), board.piece_at(square, Color::White));
}

//
// is_legal_move
//

//
// Knight
//

#[test]
fn knight_move() {
    let mut board = Board::empty();
    board.xor(Piece::Knight, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::C, File::Second);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn knight_capture() {
    let mut board = Board::empty();
    board.xor(Piece::Knight, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Pawn, Color::Black,
              Square::from_pos(Rank::C, File::Second).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::C, File::Second);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn knight_invalid() {
    let mut board = Board::empty();
    board.xor(Piece::Knight, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::C, File::Third); // invalid move
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

//
// Pawn
//

#[test]
fn pawn_single_push_white() {
    let board = Board::new();
    let src = Square::from_pos(Rank::B, File::First);
    let dst = Square::from_pos(Rank::C, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn pawn_double_push_white() {
    let board = Board::new();
    let src = Square::from_pos(Rank::B, File::First);
    let dst = Square::from_pos(Rank::D, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn pawn_single_push_black() {
    let mut board = Board::new();
    board.turn = Color::Black;
    let src = Square::from_pos(Rank::G, File::First);
    let dst = Square::from_pos(Rank::F, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn pawn_double_push_black() {
    let mut board = Board::new();
    board.turn = Color::Black;
    let src = Square::from_pos(Rank::G, File::First);
    let dst = Square::from_pos(Rank::E, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn pawn_double_push_invalid() {
    let mut board = Board::empty();
    board.xor(Piece::Pawn, Color::White,
              Square::from_pos(Rank::C, File::First).as_bb());

    let src = Square::from_pos(Rank::C, File::First);
    let dst = Square::from_pos(Rank::E, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn pawn_push_backward_invalid() {
    let mut board = Board::empty();
    board.xor(Piece::Pawn, Color::White,
              Square::from_pos(Rank::C, File::First).as_bb());

    let src = Square::from_pos(Rank::C, File::First);
    let dst = Square::from_pos(Rank::B, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn pawn_attack_invalid() {
    let board = Board::new();
    let src = Square::from_pos(Rank::B, File::First);
    let dst = Square::from_pos(Rank::C, File::Second);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn pawn_attack_valid_white() {
    let mut board = Board::new();
    board.xor(Piece::Pawn, Color::Black,
              Square::from_pos(Rank::C, File::Second).as_bb());

    let src = Square::from_pos(Rank::B, File::First);
    let dst = Square::from_pos(Rank::C, File::Second);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn pawn_attack_valid_black() {
    let mut board = Board::new();
    board.turn = Color::Black;
    board.xor(Piece::Pawn, Color::White,
              Square::from_pos(Rank::F, File::Second).as_bb());

    let src = Square::from_pos(Rank::G, File::First);
    let dst = Square::from_pos(Rank::F, File::Second);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

//
// King
//

#[test]
fn king_move() {
    let mut board = Board::empty();
    board.xor(Piece::King, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::B, File::Second);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn king_move_invalid() {
    let mut board = Board::empty();
    board.xor(Piece::King, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::C, File::Third); // can only move 1 by 1
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn king_move_in_check() {
    let mut board = Board::empty();
    board.xor(Piece::King, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Queen, Color::Black,
              Square::from_pos(Rank::B, File::Eighth).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::B, File::Second);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn king_move_next_to_enemy_king() {
    let mut board = Board::empty();
    board.xor(Piece::King, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::King, Color::Black,
              Square::from_pos(Rank::B, File::Third).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::B, File::Second);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

//
// Sliding
//

#[test]
fn queen_move_horizontal() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::A, File::Eighth);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn queen_move_vertical() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::H, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn queen_move_diagonal() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::H, File::Eighth);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn queen_move_anti_diagonal() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::Eighth).as_bb());
    let src = Square::from_pos(Rank::A, File::Eighth);
    let dst = Square::from_pos(Rank::H, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn queen_move_invalid() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::H, File::Second);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn queen_move_behind_vertical() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Pawn, Color::White,
              Square::from_pos(Rank::C, File::First).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::G, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn queen_move_behind_horizontal() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Pawn, Color::White,
              Square::from_pos(Rank::A, File::Fourth).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::A, File::Eighth);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn queen_move_behind_diagonal() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Pawn, Color::White,
              Square::from_pos(Rank::D, File::Fourth).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::H, File::Eighth);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn queen_move_behind_anti_diagonal() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::Eighth).as_bb());
    board.xor(Piece::Pawn, Color::White,
              Square::from_pos(Rank::D, File::Fifth).as_bb());

    let src = Square::from_pos(Rank::A, File::Eighth);
    let dst = Square::from_pos(Rank::H, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn queen_capture() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Pawn, Color::Black,
              Square::from_pos(Rank::G, File::First).as_bb());
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::G, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(true, result);
}

#[test]
fn queen_capture_own_piece() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Pawn, Color::White,
              Square::from_pos(Rank::G, File::First).as_bb());
    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::G, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

#[test]
fn queen_capture_behind() {
    let mut board = Board::empty();
    board.xor(Piece::Queen, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Pawn, Color::White,
              Square::from_pos(Rank::C, File::First).as_bb());
    board.xor(Piece::Pawn, Color::Black,
              Square::from_pos(Rank::G, File::First).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::G, File::First);
    let result = board.is_legal_move(ChessMove::new(src, dst));
    assert_eq!(false, result);
}

//
// Make move
//

#[test]
fn make_move_valid() {
    let board = Board::new();
    let src = Square::from_pos(Rank::B, File::Fourth);
    let dst = Square::from_pos(Rank::D, File::Fourth);
    let result = board.make_move(ChessMove::new(src, dst));
    assert_ne!(None, result);
    assert_eq!(Color::Black, result.unwrap().turn);
    assert_eq!(board.full_moves, result.unwrap().full_moves);
}

#[test]
fn make_move_valid_round() {
    let board = Board::new();
    // White opens with a pawn
    let mut result = board.make_move(ChessMove::new(
        Square::from_pos(Rank::B, File::Fourth),
        Square::from_pos(Rank::D, File::Fourth))).unwrap();

    // Black opens with a knight
    result = result.make_move(ChessMove::new(
        Square::from_pos(Rank::H, File::Second),
        Square::from_pos(Rank::F, File::Third))).unwrap();

    assert_eq!(Color::White, result.turn);
    assert_eq!(board.full_moves + 1, result.full_moves);
}

#[test]
fn make_move_king_must_leave_check() {
    let mut board = Board::empty();
    board.xor(Piece::King, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Queen, Color::Black,
              Square::from_pos(Rank::H, File::First).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::B, File::First);
    let result = board.make_move(ChessMove::new(src, dst));
    assert_eq!(None, result);
}

#[test]
fn make_move_king_leaving_check() {
    let mut board = Board::empty();
    board.xor(Piece::King, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Queen, Color::Black,
              Square::from_pos(Rank::H, File::First).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::B, File::Second);
    let result = board.make_move(ChessMove::new(src, dst));
    assert_ne!(None, result);
}

#[test]
fn make_move_capture() {
    let mut board = Board::empty();
    board.xor(Piece::Rook, Color::White,
              Square::from_pos(Rank::A, File::First).as_bb());
    board.xor(Piece::Queen, Color::Black,
              Square::from_pos(Rank::H, File::First).as_bb());

    let src = Square::from_pos(Rank::A, File::First);
    let dst = Square::from_pos(Rank::H, File::First);
    let result = board.make_move(ChessMove::new(src, dst));
    assert_ne!(None, result);
    assert!(result.unwrap().get_pieces_color(Piece::Queen, Color::Black).is_empty());
    assert!(result.unwrap().get_pieces_color(Piece::Rook, Color::White).is_not_empty());
}
