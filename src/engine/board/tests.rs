use crate::engine::board::BitBoard;

#[test]
fn new() {
    let board = BitBoard::new();
    assert_eq!(0, board.value())
}

#[test]
fn from() {
    let board = BitBoard::from(42);
    assert_eq!(42, board.value())
}

#[test]
fn default() {
    let board: BitBoard = Default::default();
    assert_eq!(0, board.value())
}

#[test]
fn eq() {
    let a = BitBoard::from(42);
    let b = BitBoard::from(42);
    assert_eq!(true, a == b)
}

#[test]
fn no_eq() {
    let a = BitBoard::new();
    let b = BitBoard::from(42);
    assert_eq!(true, a != b)
}

#[test]
fn copy() {
    let board = BitBoard::from(42);
    let other = board;
    assert_eq!(42, board.value());
    assert_eq!(42, other.value())
}

#[test]
fn clone() {
    let board = BitBoard::from(42);
    let other = board.clone();
    assert_eq!(42, board.value());
    assert_eq!(42, other.value())
}

#[test]
fn empty() {
    let board = BitBoard::empty();
    assert_eq!(0, board.value())
}

#[test]
fn universe() {
    let board = BitBoard::universe();
    assert_eq!(std::u64::MAX, board.value())
}

#[test]
fn is_empty_true() {
    let board = BitBoard::empty();
    assert_eq!(true, board.is_empty())
}

#[test]
fn is_empty_false() {
    let board = BitBoard::universe();
    assert_eq!(false, board.is_empty())
}

#[test]
fn empty_squares() {
    let board = BitBoard::universe();
    assert_eq!(BitBoard::empty().value(), board.empty_squares().value())
}

#[test]
fn empty_squares2() {
    let board = BitBoard::from(0x00FF00FF00FF00FF);
    assert_eq!(0xFF00FF00FF00FF00, board.empty_squares().value())
}

#[test]
fn and() {
    let null = BitBoard::empty() & BitBoard::universe();
    assert_eq!(0, null.value())
}

#[test]
fn and2() {
    let a = BitBoard::from(0x00000000000000FF);
    let b = BitBoard::from(0x000000000000FFFF);
    assert_eq!(a.value(), (a & b).value())
}

#[test]
fn and_u64() {
    let a = BitBoard::from(0x00000000000000FF);
    let b = 0x000000000000FFFF;
    assert_eq!(a.value(), (a & b).value())
}

#[test]
fn or() {
    let null = BitBoard::empty() | BitBoard::universe();
    assert_eq!(BitBoard::universe().value(), null.value())
}

#[test]
fn or2() {
    let a = BitBoard::from(0x0000000000FF00FF);
    let b = BitBoard::from(0x000000000000FFFF);
    assert_eq!(0x0000000000FFFFFF, (a | b).value())
}

#[test]
fn or_u64() {
    let a = BitBoard::from(0x0000000000FF00FF);
    let b = 0x000000000000FFFF;
    assert_eq!(0x0000000000FFFFFF, (a | b).value())
}

#[test]
fn xor() {
    let null = BitBoard::empty() ^ BitBoard::universe();
    assert_eq!(BitBoard::universe().value(), null.value())
}

#[test]
fn xor2() {
    let a = BitBoard::from(0x0000000000FF00FF);
    let b = BitBoard::from(0x000000000000FFFF);
    assert_eq!(0x0000000000FFFF00, (a ^ b).value())
}

#[test]
fn xor_u64() {
    let a = BitBoard::from(0x0000000000FF00FF);
    let b = 0x000000000000FFFF;
    assert_eq!(0x0000000000FFFF00, (a ^ b).value())
}

#[test]
fn shl_usize() {
    let a = BitBoard::from(0x0000000000FF00FF);
    assert_eq!(0x00000000FF00FF00, (a << 8).value())
}

#[test]
fn shr_usize() {
    let a = BitBoard::from(0x00000000FF00FF00);
    assert_eq!(0x0000000000FF00FF, (a >> 8).value())
}

#[test]
fn complement() {
    let universe = BitBoard::empty().complement();
    assert_eq!(BitBoard::universe().value(), universe.value())
}

#[test]
fn complement2() {
    let board = BitBoard::from(0x00FF00FF00FF00FF);
    assert_eq!(0xFF00FF00FF00FF00, board.complement().value())
}

#[test]
fn rotate_left() {
    let board = BitBoard::from(0xFFFF000000000000);
    assert_eq!(0xFF000000000000FF, board.rotate_left(8).value())
}

#[test]
fn rotate_right() {
    let board = BitBoard::from(0x000000000000FF);
    assert_eq!(0xFF00000000000000, board.rotate_right(8).value())
}

#[test]
fn north_one() {
    let board = BitBoard::from(0x000000000000FF);
    assert_eq!(0x000000000000FF00, board.north_one().value())
}

#[test]
fn north_one2() {
    let board = BitBoard::from(0xFF00000000000000);
    assert_eq!(0x0000000000000000, board.north_one().value())
}

#[test]
fn south_one() {
    let board = BitBoard::from(0x000000000000FF00);
    assert_eq!(0x00000000000000FF, board.south_one().value())
}

#[test]
fn south_one_outside() {
    let board = BitBoard::from(0x00000000000000FF);
    assert_eq!(0x0000000000000000, board.south_one().value())
}

#[test]
fn east_one() {
    let board = BitBoard::from(0x0000000000000001);
    assert_eq!(0x0000000000000002, board.east_one().value())
}

#[test]
fn east_one_outside() {
    let board = BitBoard::from(0x0000000000000080);
    assert_eq!(0x0000000000000000, board.east_one().value())
}

#[test]
fn no_ea_one() {
    let board = BitBoard::from(0x0000000000000001);
    assert_eq!(0x0000000000000200, board.no_ea_one().value())
}

#[test]
fn no_ea_one_outside() {
    let board = BitBoard::from(0x0000000000000080);
    assert_eq!(0x0000000000000000, board.no_ea_one().value())
}

#[test]
fn so_ea_one() {
    let board = BitBoard::from(0x0000000000000100);
    assert_eq!(0x0000000000000002, board.so_ea_one().value())
}

#[test]
fn so_ea_one_outside() {
    let board = BitBoard::from(0x0000000000008000);
    assert_eq!(0x0000000000000000, board.so_ea_one().value())
}

#[test]
fn west_one() {
    let board = BitBoard::from(0x0000000000000002);
    assert_eq!(0x0000000000000001, board.west_one().value())
}

#[test]
fn west_one_outside() {
    let board = BitBoard::from(0x0000000000000001);
    assert_eq!(0x0000000000000000, board.west_one().value())
}

#[test]
fn no_we_one() {
    let board = BitBoard::from(0x0000000000000002);
    assert_eq!(0x0000000000000100, board.no_we_one().value())
}

#[test]
fn no_we_one_outside() {
    let board = BitBoard::from(0x0100000000000000);
    assert_eq!(0x0000000000000000, board.no_we_one().value())
}

#[test]
fn so_we_one() {
    let board = BitBoard::from(0x0000000000000200);
    assert_eq!(0x0000000000000001, board.so_we_one().value())
}

#[test]
fn so_we_one_outside() {
    let board = BitBoard::from(0x0100000000000004);
    assert_eq!(0x0000000000000000, board.so_we_one().value())
}

