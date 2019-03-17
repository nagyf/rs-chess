// TODO: remove this
#![allow(unused)]

use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::piece::Color;
use crate::engine::board::square::constants::{RANK_4, RANK_5};

pub fn push_targets(color: Color, pawns: BitBoard, empty: BitBoard) -> BitBoard {
    match color {
        Color::White => single_push_targets_white(pawns, empty) | double_push_targets_white(pawns, empty),
        Color::Black => single_push_targets_black(pawns, empty) | double_push_targets_black(pawns, empty),
    }
}

pub fn single_push_targets(color: Color, pawns: BitBoard, empty: BitBoard) -> BitBoard {
    match color {
        Color::White => single_push_targets_white(pawns, empty),
        Color::Black => single_push_targets_black(pawns, empty),
    }
}

pub fn double_push_targets(color: Color, pawns: BitBoard, empty: BitBoard) -> BitBoard {
    match color {
        Color::White => double_push_targets_white(pawns, empty),
        Color::Black => double_push_targets_black(pawns, empty),
    }
}

pub fn able_to_push(color: Color, pawns: BitBoard, empty: BitBoard) -> BitBoard {
    able_to_single_push(color, pawns, empty) | able_to_double_push(color, pawns, empty)
}

pub fn able_to_single_push(color: Color, pawns: BitBoard, empty: BitBoard) -> BitBoard {
    match color {
        Color::White => able_to_push_white(pawns, empty),
        Color::Black => able_to_push_black(pawns, empty),
    }
}

pub fn able_to_double_push(color: Color, pawns: BitBoard, empty: BitBoard) -> BitBoard {
    match color {
        Color::White => able_to_double_push_white(pawns, empty),
        Color::Black => able_to_double_push_black(pawns, empty),
    }
}

fn single_push_targets_white(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    pawns.north_one() & empty
}

fn double_push_targets_white(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    let single_pushes = single_push_targets_white(pawns, empty);
    single_pushes.north_one() & empty & RANK_4
}

fn single_push_targets_black(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    pawns.south_one() & empty
}

fn double_push_targets_black(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    let single_pushes = single_push_targets_black(pawns, empty);
    single_pushes.south_one() & empty & RANK_5
}

fn able_to_push_white(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    empty.south_one() & pawns
}

fn able_to_double_push_white(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    let empty_rank_3 = (empty & RANK_4).south_one() & empty;
    able_to_push_white(pawns, empty_rank_3)
}

fn able_to_push_black(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    empty.north_one() & pawns
}

fn able_to_double_push_black(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    let empty_rank_6 = (empty & RANK_5).north_one() & empty;
    able_to_push_black(pawns, empty_rank_6)
}

//
// Attacks
//

pub fn any_valid_attack(color: Color, pawns: BitBoard, other_pieces: BitBoard) -> BitBoard {
    match color {
        Color::White => pawn_any_attack_white(pawns) & other_pieces,
        Color::Black => pawn_any_attack_black(pawns) & other_pieces
    }
}

pub fn any_attack(color: Color, pawns: BitBoard) -> BitBoard {
    match color {
        Color::White => pawn_any_attack_white(pawns),
        Color::Black => pawn_any_attack_black(pawns)
    }
}

fn pawn_east_attacks_white(pawns: BitBoard) -> BitBoard {
    pawns.no_ea_one()
}

fn pawn_west_attacks_white(pawns: BitBoard) -> BitBoard {
    pawns.no_we_one()
}

fn pawn_any_attack_white(pawns: BitBoard) -> BitBoard {
    pawn_east_attacks_white(pawns) | pawn_west_attacks_white(pawns)
}

fn pawn_double_attack_white(pawns: BitBoard) -> BitBoard {
    pawn_east_attacks_white(pawns) & pawn_west_attacks_white(pawns)
}

fn pawn_single_attack_white(pawns: BitBoard) -> BitBoard {
    pawn_east_attacks_white(pawns) ^ pawn_west_attacks_white(pawns)
}

fn pawn_east_attacks_black(pawns: BitBoard) -> BitBoard {
    pawns.so_ea_one()
}

fn pawn_west_attacks_black(pawns: BitBoard) -> BitBoard {
    pawns.so_we_one()
}

fn pawn_any_attack_black(pawns: BitBoard) -> BitBoard {
    pawn_east_attacks_black(pawns) | pawn_west_attacks_black(pawns)
}

fn pawn_double_attack_black(pawns: BitBoard) -> BitBoard {
    pawn_east_attacks_black(pawns) & pawn_west_attacks_black(pawns)
}

fn pawn_single_attack_black(pawns: BitBoard) -> BitBoard {
    pawn_east_attacks_black(pawns) ^ pawn_west_attacks_black(pawns)
}

//
// Captures
//

pub fn able_to_capture(color: Color, pawns: BitBoard, other_pieces: BitBoard) -> BitBoard {
    match color {
        Color::White => able_to_capture_any_white(pawns, other_pieces),
        Color::Black => able_to_capture_any_black(pawns, other_pieces)
    }
}

fn able_to_capture_east_white(white_pawns: BitBoard, black_pieces: BitBoard) -> BitBoard {
    white_pawns & pawn_west_attacks_black(black_pieces)
}

fn able_to_capture_west_white(white_pawns: BitBoard, black_pieces: BitBoard) -> BitBoard {
    white_pawns & pawn_east_attacks_black(black_pieces)
}

fn able_to_capture_any_white(white_pawns: BitBoard, black_pieces: BitBoard) -> BitBoard {
    white_pawns & pawn_any_attack_black(black_pieces)
}

fn able_to_capture_east_black(black_pawns: BitBoard, white_pieces: BitBoard) -> BitBoard {
    black_pawns & pawn_west_attacks_white(white_pieces)
}

fn able_to_capture_west_black(black_pawns: BitBoard, white_pieces: BitBoard) -> BitBoard {
    black_pawns & pawn_east_attacks_white(white_pieces)
}

fn able_to_capture_any_black(black_pawns: BitBoard, white_pieces: BitBoard) -> BitBoard {
    black_pawns & pawn_any_attack_white(white_pieces)
}