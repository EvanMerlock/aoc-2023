mod question_one;
mod question_two;

pub use self::question_one::question_one;
pub use self::question_two::question_two;

use std::{io::{BufReader, BufRead}, fs::File, cmp::Ordering, fmt::Debug};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard
}

impl Debug for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FiveKind => write!(f, "FiveKind"),
            Self::FourKind => write!(f, "FourKind"),
            Self::FullHouse => write!(f, "FullHouse"),
            Self::ThreeKind => write!(f, "ThreeKind"),
            Self::TwoPair => write!(f, "TwoPair"),
            Self::OnePair => write!(f, "OnePair"),
            Self::HighCard => write!(f, "HighCard"),
        }
    }
}
