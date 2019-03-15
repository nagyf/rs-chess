#[cfg(test)]
mod rank_test {
    use crate::engine::board::square::Rank;

    #[test]
    fn from_id() {
        assert_eq!(Some(Rank::A), Rank::from_id("a"));
        assert_eq!(Some(Rank::B), Rank::from_id("b"));
        assert_eq!(Some(Rank::C), Rank::from_id("c"));
        assert_eq!(Some(Rank::D), Rank::from_id("d"));
        assert_eq!(Some(Rank::E), Rank::from_id("e"));
        assert_eq!(Some(Rank::F), Rank::from_id("f"));
        assert_eq!(Some(Rank::G), Rank::from_id("g"));
        assert_eq!(Some(Rank::H), Rank::from_id("h"));
    }

    #[test]
    fn from_id_err() {
        assert_eq!(None, Rank::from_id(""));
        assert_eq!(None, Rank::from_id("i"));
        assert_eq!(None, Rank::from_id("1"));
    }

    #[test]
    fn from_index() {
        assert_eq!(Some(Rank::A), Rank::from_index(1));
        assert_eq!(Some(Rank::B), Rank::from_index(2));
        assert_eq!(Some(Rank::C), Rank::from_index(3));
        assert_eq!(Some(Rank::D), Rank::from_index(4));
        assert_eq!(Some(Rank::E), Rank::from_index(5));
        assert_eq!(Some(Rank::F), Rank::from_index(6));
        assert_eq!(Some(Rank::G), Rank::from_index(7));
        assert_eq!(Some(Rank::H), Rank::from_index(8));
    }

    #[test]
    fn from_index_err() {
        assert_eq!(None, Rank::from_index(0));
        assert_eq!(None, Rank::from_index(9));
    }

    #[test]
    fn to_index() {
        assert_eq!(1, Rank::A.to_index());
        assert_eq!(2, Rank::B.to_index());
        assert_eq!(3, Rank::C.to_index());
        assert_eq!(4, Rank::D.to_index());
        assert_eq!(5, Rank::E.to_index());
        assert_eq!(6, Rank::F.to_index());
        assert_eq!(7, Rank::G.to_index());
        assert_eq!(8, Rank::H.to_index());
    }
}

#[cfg(test)]
mod file_test {
    use crate::engine::board::square::File;

    #[test]
    fn from_index() {
        assert_eq!(Some(File::First), File::from_index(1));
        assert_eq!(Some(File::Second), File::from_index(2));
        assert_eq!(Some(File::Third), File::from_index(3));
        assert_eq!(Some(File::Fourth), File::from_index(4));
        assert_eq!(Some(File::Fifth), File::from_index(5));
        assert_eq!(Some(File::Sixth), File::from_index(6));
        assert_eq!(Some(File::Seventh), File::from_index(7));
        assert_eq!(Some(File::Eighth), File::from_index(8));
    }

    #[test]
    fn from_index_err() {
        assert_eq!(None, File::from_index(0));
        assert_eq!(None, File::from_index(9));
    }

    #[test]
    fn to_index() {
        assert_eq!(1, File::First.to_index());
        assert_eq!(2, File::Second.to_index());
        assert_eq!(3, File::Third.to_index());
        assert_eq!(4, File::Fourth.to_index());
        assert_eq!(5, File::Fifth.to_index());
        assert_eq!(6, File::Sixth.to_index());
        assert_eq!(7, File::Seventh.to_index());
        assert_eq!(8, File::Eighth.to_index());
    }
}

#[cfg(test)]
mod square_test {
    use crate::engine::board::square::{File, Rank, Square};

    #[test]
    fn square_default() {
        assert_eq!(Square(0), Square::default())
    }

    #[test]
    fn new() {
        assert_eq!(42, Square::new(42).0)
    }

    #[test]
    fn from_pos() {
        assert_eq!(0, Square::from_pos(Rank::A, File::First).0);
        assert_eq!(8, Square::from_pos(Rank::B, File::First).0);
        assert_eq!(63, Square::from_pos(Rank::H, File::Eighth).0);
    }

    #[test]
    fn get_rank() {
        assert_eq!(Rank::A, Square::from_pos(Rank::A, File::First).get_rank());
        assert_eq!(Rank::H, Square::from_pos(Rank::H, File::Eighth).get_rank());
    }

    #[test]
    fn get_file() {
        assert_eq!(File::First, Square::from_pos(Rank::A, File::First).get_file());
        assert_eq!(File::Eighth, Square::from_pos(Rank::H, File::Eighth).get_file());
    }
}