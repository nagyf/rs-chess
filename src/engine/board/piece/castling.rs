//! This module implements Castling Right related logic.

/// This enum represents the castling right of the player.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CastlingRight {
    /// Queen side castling is allowed.
    QueenSide,
    /// King side castling is allowed.
    KingSide,
    /// Castling is allowed on both queen and king side.
    BothSide,
    /// Castling is not allowed.
    NoRight,
}

impl CastlingRight {
    /// Merges castling rights.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(CastlingRight::BothSide, CastlingRight::QueenSide.merge(CastlingRight.KingSide));
    /// ```
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
