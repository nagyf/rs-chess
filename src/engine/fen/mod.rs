use regex::Regex;

use crate::engine::board::{Board, piece};
use crate::engine::board::square::{File, Rank, Square};
use crate::engine::board::piece::{CastlingRight, Color};

#[cfg(test)]
mod tests;

pub const INITIAL_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Eq, PartialEq)]
pub enum FENParseError {
    FENPiece(String),
    FENSideToMove(String),
    FENPieceType(String),
    FENPieceColor(String),
    FENHalfMove(String),
    FENFullMove(String),
    FENEnPassant(String),
    FENCastlingAbility(String),
}

struct FENPiece {
    piece_type: piece::Piece,
    color: piece::Color,
    file: File,
    rank: Rank,
}

/// Parse a FEN string and produce a Board
pub fn from_fen(input: &str) -> Result<Board, FENParseError> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let pieces = parse_pieces(parts[0])?;
    let side_to_move = parse_side_to_move(parts[1])?;
    let catling_rights = parse_castling_rights(parts[2])?;
    let en_passant = parse_en_passant(parts[3])?;
    let half_moves = parse_half_moves(parts[4])?;
    let full_moves = parse_full_moves(parts[5])?;
    let mut board = Board::empty();
    board.turn = side_to_move;
    board.half_moves = half_moves;
    board.full_moves = full_moves;
    board.en_passant = en_passant;
    board.castling_rights = catling_rights;

    for piece in pieces {
        board = board.add_piece(piece.piece_type, piece.color, piece.rank, piece.file);
    }

    Ok(board)
}

fn parse_pieces(input: &str) -> Result<Vec<FENPiece>, FENParseError> {
    let ranks: Vec<&str> = input.split("/").collect();
    let mut pieces = Vec::new();

    for i in 0..ranks.len() {
        let mut parsed = parse_rank(ranks[i], 8 - i as u8)?;
        pieces.append(&mut parsed);
    }

    Ok(pieces)
}

fn parse_rank(rank: &str, rank_num: u8) -> Result<Vec<FENPiece>, FENParseError> {
    let mut pieces = Vec::new();
    let mut file: u8 = 1;

    let is_number = Regex::new(r"^[1-8]$").unwrap();
    let is_piece = Regex::new(r"^[PNBRQKpnbrqk]$").unwrap();

    for ch in rank.chars() {
        let ch_str = ch.to_string();
        if is_number.is_match(&ch_str) {
            let skip: u8 = ch_str.parse().unwrap();
            file += skip;
        } else if is_piece.is_match(&ch_str) {
            let piece_type = parse_piece_type(&ch_str)?;
            let color = parse_piece_color(&ch_str)?;
            pieces.push(FENPiece {
                piece_type,
                color,
                file: File::from_index(file).unwrap(),
                rank: Rank::from_index(rank_num).unwrap(),
            });
            file += 1;
        } else {
            return Err(FENParseError::FENPiece(format!("Unable to parse rank: {}", rank)))
        }
    }

    Ok(pieces)
}

fn parse_piece_type(input: &str) -> Result<piece::Piece, FENParseError> {
    match input.to_ascii_lowercase().trim() {
        "p" => Ok(piece::Piece::Pawn),
        "n" => Ok(piece::Piece::Knight),
        "b" => Ok(piece::Piece::Bishop),
        "r" => Ok(piece::Piece::Rook),
        "q" => Ok(piece::Piece::Queen),
        "k" => Ok(piece::Piece::King),
        _ => Err(FENParseError::FENPieceType(format!("Unknown piece type: {}", input)))
    }
}

fn parse_piece_color(input: &str) -> Result<piece::Color, FENParseError> {
    let is_white_piece = Regex::new(r"^[PNBRQK]$").unwrap();
    let is_black_piece = Regex::new(r"^[pnbrqk]$").unwrap();

    if is_white_piece.is_match(input) {
        Ok(piece::Color::White)
    } else if is_black_piece.is_match(input) {
        Ok(piece::Color::Black)
    } else {
        Err(FENParseError::FENPieceColor(format!("Unknown color: {}", input)))
    }
}

fn parse_side_to_move(input: &str) -> Result<piece::Color, FENParseError> {
    match input {
        "w" => Ok(piece::Color::White),
        "b" => Ok(piece::Color::Black),
        _ => {
            let msg = format!("Unable to parse side to chessmove: {}", input);
            Err(FENParseError::FENSideToMove(msg))
        }
    }
}

fn parse_half_moves(input: &str) -> Result<u16, FENParseError> {
    let regex = Regex::new("^\\d+$").unwrap();

    if regex.is_match(input) {
        Ok(input.parse().unwrap())
    } else {
        Err(FENParseError::FENHalfMove(format!("Unable to parse half chessmove clock: {}", input)))
    }
}

fn parse_full_moves(input: &str) -> Result<u16, FENParseError> {
    let regex = Regex::new("^\\d+$").unwrap();

    if regex.is_match(input) {
        Ok(input.parse().unwrap())
    } else {
        Err(FENParseError::FENFullMove(format!("Unable to parse full chessmove clock: {}", input)))
    }
}

fn parse_en_passant(input: &str) -> Result<Option<Square>, FENParseError> {
    let regex = Regex::new("^(-|[a-h][1-8])$").unwrap();

    if regex.is_match(input) {
        match input {
            "-" => Ok(None),
            ss => {
                let chars: Vec<char> = ss.chars().collect();
                let rank = Rank::from_id(&chars.get(0).unwrap().to_string()).unwrap();
                let file = File::from_index(chars.get(1).unwrap().to_string().parse().unwrap()).unwrap();
                Ok(Some(Square::from_pos(rank, file)))
            }
        }
    } else {
        Err(FENParseError::FENEnPassant(format!("Unable to parse en passant target: {}", input)))
    }
}

fn parse_castling_rights(input: &str) -> Result<[CastlingRight; piece::NUM_COLORS], FENParseError> {
    let regex = Regex::new("^(-|[KkQq]+)$").unwrap();

    let mut white = CastlingRight::NoRight;
    let mut black = CastlingRight::NoRight;

    if regex.is_match(input) {
        for ch in input.chars() {
            match ch {
                'K' => white = white.merge(CastlingRight::KingSide),
                'Q' => white = white.merge(CastlingRight::QueenSide),
                'k' => black = black.merge(CastlingRight::KingSide),
                'q' => black = black.merge(CastlingRight::QueenSide),
                _ => {}
            }
        }

        let mut result: [CastlingRight; piece::NUM_COLORS] = [CastlingRight::NoRight, CastlingRight::NoRight];
        result[Color::White.to_index()] = white;
        result[Color::Black.to_index()] = black;
        Ok(result)
    } else {
        Err(FENParseError::FENCastlingAbility(format!("Unable to parse castling ability: {}", input)))
    }
}