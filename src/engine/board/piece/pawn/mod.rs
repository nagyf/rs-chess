// TODO: remove this
#![allow(unused)]

use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::piece::Color;
use crate::engine::board::square::constants::{RANK_4, RANK_5};

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

pub fn pawns_able_to_push(color: Color, pawns: BitBoard, empty: BitBoard) -> BitBoard {
    match color {
        Color::White => pawns_able_to_push_white(pawns, empty),
        Color::Black => pawns_able_to_push_black(pawns, empty),
    }
}

pub fn pawns_able_to_double_push(color: Color, pawns: BitBoard, empty: BitBoard) -> BitBoard {
    match color {
        Color::White => pawns_able_to_double_push_white(pawns, empty),
        Color::Black => pawns_able_to_double_push_black(pawns, empty),
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

fn pawns_able_to_push_white(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    empty.south_one() & pawns
}

fn pawns_able_to_double_push_white(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    let empty_rank_3 = (empty & RANK_4).south_one() & empty;
    pawns_able_to_push_white(pawns, empty_rank_3)
}

fn pawns_able_to_push_black(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    empty.north_one() & pawns
}

fn pawns_able_to_double_push_black(pawns: BitBoard, empty: BitBoard) -> BitBoard {
    let empty_rank_6 = (empty & RANK_5).north_one() & empty;
    pawns_able_to_push_black(pawns, empty_rank_6)
}

//
// Attacks
//

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

fn pawns_able2capture_east_white(white_pawns: BitBoard, black_pieces: BitBoard) -> BitBoard {
    white_pawns & pawn_west_attacks_black(black_pieces)
}

fn pawns_able2capture_west_white(white_pawns: BitBoard, black_pieces: BitBoard) -> BitBoard {
    white_pawns & pawn_east_attacks_black(black_pieces)
}

fn pawns_able2capture_any_white(white_pawns: BitBoard, black_pieces: BitBoard) -> BitBoard {
    white_pawns & pawn_any_attack_black(black_pieces)
}