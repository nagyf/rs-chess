#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rank {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum File {
    First = 1,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Square(u8);

impl Rank {
    pub fn from_id(id: &str) -> Option<Rank> {
        match id {
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

    pub fn from_index(index: u8) -> Option<Rank> {
        match index {
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

    pub fn to_index(&self) -> u8 {
        *self as u8
    }
}

impl File {
    pub fn from_index(index: u8) -> Option<File> {
        match index {
            1 => Some(File::First),
            2 => Some(File::Second),
            3 => Some(File::Third),
            4 => Some(File::Fourth),
            5 => Some(File::Fifth),
            6 => Some(File::Sixth),
            7 => Some(File::Seventh),
            8 => Some(File::Eighth),
            _ => None
        }
    }

    pub fn to_index(&self) -> u8 {
        *self as u8
    }
}

impl Default for Square {
    fn default() -> Self {
        Square::new(0)
    }
}

impl Square {
    pub fn new(index: u8) -> Square {
        Square(index)
    }

    pub fn from_pos(rank: Rank, file: File) -> Square{
        let rank = rank.to_index() - 1;
        let file = file.to_index() - 1;
        Square::new(rank * 8 + file)
    }

    pub fn get_rank(&self) -> Rank {
        Rank::from_index(self.0 / 8 + 1).unwrap()
    }

    pub fn get_file(&self) -> File {
        File::from_index(self.0 % 8 + 1).unwrap()
    }
}