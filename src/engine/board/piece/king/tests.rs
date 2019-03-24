use crate::engine::board::bitboard::BitBoard;
use crate::engine::board::piece::king::attack_targets;
use crate::engine::board::square::{File, Rank, Square};

//
// Test with 1 king in the corner
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// x x . . . . . .
// K x . . . . . .
//
#[test]
fn king_in_corner() {
    let king = BitBoard::from(0x0000000000000001);
    let attacks = attack_targets(king);
    assert_eq!(0x0000000000000302, attacks.0);
}


//
// Test with 2 kings in the corners
//
// . . . . . . x K
// . . . . . . x x
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// x x . . . . . .
// K x . . . . . .
//
#[test]
fn kings_in_corners() {
    let kings = Square::from_pos(Rank::A, File::First).as_bb()
        | Square::from_pos(Rank::H, File::Eighth).as_bb();

    let attacks = attack_targets(kings);
    assert_eq!(0x40C0000000000302, attacks.0);
}

//
// Test king in middle
//
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . x x x . . .
// . . x K x . . .
// . . x x x . . .
// . . . . . . . .
// . . . . . . . .
//
#[test]
fn king_in_middle() {
    let king = Square::from_pos(Rank::D, File::Fourth).as_bb();
    let attacks = attack_targets(king);
    assert_eq!(0x0000001C141C0000, attacks.0);
}
