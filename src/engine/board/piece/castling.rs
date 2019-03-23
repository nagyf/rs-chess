#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CastlingRight {
    QueenSide,
    KingSide,
    BothSide,
    NoRight,
}

impl CastlingRight {
    pub fn merge(&self, other: CastlingRight) -> CastlingRight {
        match *self {
            CastlingRight::NoRight => other,
            CastlingRight::BothSide => *self,
            _ => {
                if *self == other {
                    other
                } else {
                    match other {
                        CastlingRight::QueenSide | CastlingRight::KingSide => CastlingRight::BothSide,
                        CastlingRight::NoRight => *self,
                        CastlingRight::BothSide => CastlingRight::BothSide
                    }
                }
            }
        }
    }
}