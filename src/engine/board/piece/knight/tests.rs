use crate::engine::board::piece::knight::attack_targets;
use crate::engine::board::square::{File, Rank, Square};

//
// Test knight in middle
//
// . . . . . . . .
// . . . . . . . .
// . . x . x . . .
// . x . . . x . .
// . . . N . . . .
// . x . . . x . .
// . . x . x . . .
// . . . . . . . .
//
#[test]
fn knight_in_middle() {
    let knight = Square::from_pos(Rank::D, File::Fourth).as_bb();
    let attacks = attack_targets(knight);
    assert_eq!(0x0000142200221400, attacks.0);
}

//
// Test knights in the corners
//
// . . . . . . . N
// . . . . . x . .
// . . . . . . x .
// . . . . . . . .
// . . . . . . . .
// . x . . . . . .
// . . x . . . . .
// N . . . . . . .
//
#[test]
fn knights_in_corners() {
    let knights = Square::from_pos(Rank::A, File::First).as_bb()
        | Square::from_pos(Rank::H, File::Eighth).as_bb();
    let attacks = attack_targets(knights);
    assert_eq!(0x0020400000020400, attacks.0);
}
