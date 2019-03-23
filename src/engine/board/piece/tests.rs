#[cfg(test)]
mod color {
    use crate::engine::board::piece::color::Color;

    #[test]
    fn to_index() {
        assert_eq!(0, Color::White.to_index());
        assert_eq!(1, Color::Black.to_index());
    }

    #[test]
    fn not() {
        assert_eq!(Color::Black, !Color::White);
        assert_eq!(Color::White, !Color::Black);
    }

    #[test]
    fn display() {
        assert_eq!("w", Color::White.to_string());
        assert_eq!("b", Color::Black.to_string());
    }
}

#[cfg(test)]
mod piece {
    use crate::engine::board::piece::Piece;

    #[test]
    fn to_index() {
        assert_eq!(0, Piece::Pawn.to_index());
        assert_eq!(1, Piece::Rook.to_index());
        assert_eq!(2, Piece::Knight.to_index());
        assert_eq!(3, Piece::Bishop.to_index());
        assert_eq!(4, Piece::King.to_index());
        assert_eq!(5, Piece::Queen.to_index());
    }
}

#[cfg(test)]
mod castling_right {
    use crate::engine::board::piece::castling::CastlingRight;

    #[test]
    fn no_no() {
        assert_eq!(CastlingRight::NoRight, CastlingRight::NoRight.merge(CastlingRight::NoRight));
    }

    #[test]
    fn both_both() {
        assert_eq!(CastlingRight::BothSide, CastlingRight::BothSide.merge(CastlingRight::BothSide));
    }

    #[test]
    fn no_both() {
        assert_eq!(CastlingRight::BothSide, CastlingRight::BothSide.merge(CastlingRight::NoRight));
        assert_eq!(CastlingRight::BothSide, CastlingRight::NoRight.merge(CastlingRight::BothSide));
    }

    #[test]
    fn no_king() {
        assert_eq!(CastlingRight::KingSide, CastlingRight::NoRight.merge(CastlingRight::KingSide));
        assert_eq!(CastlingRight::KingSide, CastlingRight::KingSide.merge(CastlingRight::NoRight));
    }

    #[test]
    fn no_queen() {
        assert_eq!(CastlingRight::QueenSide, CastlingRight::NoRight.merge(CastlingRight::QueenSide));
        assert_eq!(CastlingRight::QueenSide, CastlingRight::QueenSide.merge(CastlingRight::NoRight));
    }

    #[test]
    fn king_queen() {
        assert_eq!(CastlingRight::BothSide, CastlingRight::KingSide.merge(CastlingRight::QueenSide));
        assert_eq!(CastlingRight::BothSide, CastlingRight::QueenSide.merge(CastlingRight::KingSide));
    }

    #[test]
    fn king_both() {
        assert_eq!(CastlingRight::BothSide, CastlingRight::KingSide.merge(CastlingRight::BothSide));
        assert_eq!(CastlingRight::BothSide, CastlingRight::BothSide.merge(CastlingRight::KingSide));
    }

    #[test]
    fn queen_both() {
        assert_eq!(CastlingRight::BothSide, CastlingRight::QueenSide.merge(CastlingRight::BothSide));
        assert_eq!(CastlingRight::BothSide, CastlingRight::BothSide.merge(CastlingRight::QueenSide));
    }

    #[test]
    fn king_no() {
        assert_eq!(CastlingRight::KingSide, CastlingRight::KingSide.merge(CastlingRight::NoRight));
        assert_eq!(CastlingRight::KingSide, CastlingRight::NoRight.merge(CastlingRight::KingSide));
    }

    #[test]
    fn queen_no() {
        assert_eq!(CastlingRight::QueenSide, CastlingRight::QueenSide.merge(CastlingRight::NoRight));
        assert_eq!(CastlingRight::QueenSide, CastlingRight::NoRight.merge(CastlingRight::QueenSide));
    }
}