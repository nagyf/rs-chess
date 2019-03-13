#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rank {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum File {
    F1 = 1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8
}

pub fn file_as_number(file: File) -> u8 {
    file as u8
}

pub fn number_as_file(file: u8) -> Option<File> {
    match file {
        1 => Some(File::F1),
        2 => Some(File::F2),
        3 => Some(File::F3),
        4 => Some(File::F4),
        5 => Some(File::F5),
        6 => Some(File::F6),
        7 => Some(File::F7),
        8 => Some(File::F8),
        _ => None
    }
}

pub fn rank_as_number(rank: Rank) -> u8 {
    rank as u8
}

pub fn number_as_rank(rank: u8) -> Option<Rank> {
    match rank {
        1 => Some(Rank::A),
        2 => Some(Rank::B),
        3 => Some(Rank::C),
        4 => Some(Rank::D),
        5 => Some(Rank::E),
        6 => Some(Rank::F),
        7 => Some(Rank::G),
        8 => Some(Rank::H),
        _ => None
    }
}

pub fn letter_as_rank(rank: &str) -> Option<Rank> {
    match rank {
        "a" => Some(Rank::A),
        "b" => Some(Rank::B),
        "c" => Some(Rank::C),
        "d" => Some(Rank::D),
        "e" => Some(Rank::E),
        "f" => Some(Rank::F),
        "g" => Some(Rank::G),
        "h" => Some(Rank::H),
        _ => None
    }
}