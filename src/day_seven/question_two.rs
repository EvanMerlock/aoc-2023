use std::{io::{BufReader, BufRead}, fs::File, cmp::Ordering, fmt::Debug};

use super::HandType;


#[derive(PartialOrd, Eq, Ord, Clone, Copy)]
enum Card {
    A = 13,
    K = 12,
    Q = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    J = 1,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        // if it's a J, it's equal always
        if core::mem::discriminant(self) == core::mem::discriminant(&Card::J) || core::mem::discriminant(other) == core::mem::discriminant(&Card::J){
            true
        } else {
            core::mem::discriminant(self) == core::mem::discriminant(other)
        }
    }
}

impl Card {
    fn parse(c: char) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            
            _ => panic!("bad card!!!")
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::K => write!(f, "K"),
            Self::Q => write!(f, "Q"),
            Self::J => write!(f, "J"),
            Self::T => write!(f, "T"),
            Self::Nine => write!(f, "9"),
            Self::Eight => write!(f, "8"),
            Self::Seven => write!(f, "7"),
            Self::Six => write!(f, "6"),
            Self::Five => write!(f, "5"),
            Self::Four => write!(f, "4"),
            Self::Three => write!(f, "3"),
            Self::Two => write!(f, "2"),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    ty: HandType,
    cards: Vec<Card>,
    bid: usize
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.ty.partial_cmp(&other.ty) {
            Some(Ordering::Equal) => {
                for (position, my_card) in self.cards.iter().enumerate() {
                    let other_card = &other.cards[position];

                    match my_card.partial_cmp(other_card) {
                        Some(Ordering::Equal) => {
                            continue;
                        },
                        ord => return ord,
                    }
                }
                None
            }
            ord => ord,
        }
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}{:?}{:?}{:?}{:?}] - {:?}", self.cards[0], self.cards[1], self.cards[2],self.cards[3],self.cards[4], self.ty)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("this should never happen")
    }
} 

fn compute_hand_type(cards: &[Card]) -> HandType {

    let mut card_check = cards.to_owned();
    card_check.sort();
    
    // first check 5 of a kind
    let is_five_kind: bool = {
        card_check[0] == card_check[1] && card_check[1] == card_check[2] && card_check[2] == card_check[3] && card_check[3] == card_check[4]
    };

    if is_five_kind {
        println!("[{:?}{:?}{:?}{:?}{:?}] - FiveKind", card_check[0], card_check[1], card_check[2], card_check[3], card_check[4]);
        return HandType::FiveKind
    };

    let is_four_kind = {
        (card_check[0] == card_check[1] && card_check[1] == card_check[2] && card_check[2] == card_check[3])
        || (card_check[1] == card_check[2] && card_check[2] == card_check[3] && card_check[3] == card_check[4])
        || (card_check[0] == card_check[1] && card_check[1] == card_check[2] && card_check[2] == card_check[4])
    };

    if is_four_kind {
        println!("[{:?}{:?}{:?}{:?}{:?}] - FourKind", card_check[0], card_check[1], card_check[2], card_check[3], card_check[4]);
        return HandType::FourKind
    };

    let is_full_house = {
        ((card_check[0] == card_check[1] && card_check[1] == card_check[2]) && (card_check[3] == card_check[4]))
        || ((card_check[2] == card_check[3] && card_check[3] == card_check[4]) && (card_check[0] == card_check[1]))
        || ((card_check[1] == card_check[2] && card_check[2] == card_check[3]) && (card_check[0] == card_check[4]))
    };

    if is_full_house {
        println!("[{:?}{:?}{:?}{:?}{:?}] - FullHouse", card_check[0], card_check[1], card_check[2], card_check[3], card_check[4]);
        return HandType::FullHouse
    };

    let is_three_kind = {
        (card_check[0] == card_check[1] && card_check[1] == card_check[2])
        || (card_check[1] == card_check[2] && card_check[2] == card_check[3])
        || (card_check[2] == card_check[3] && card_check[3] == card_check[4])
        || (card_check[0] == card_check[1] && card_check[1] == card_check[4])
    };

    if is_three_kind {
        println!("[{:?}{:?}{:?}{:?}{:?}] - ThreeKind", card_check[0], card_check[1], card_check[2], card_check[3], card_check[4]);
        return HandType::ThreeKind
    };

    let is_two_pair = {
        (card_check[0] == card_check[1] && card_check[2] == card_check[3])
        || (card_check[1] == card_check[2] && card_check[3] == card_check[4])
        || (card_check[0] == card_check[1] && card_check[3] == card_check[4])
    };

    if is_two_pair {
        println!("[{:?}{:?}{:?}{:?}{:?}] - TwoPair", card_check[0], card_check[1], card_check[2], card_check[3], card_check[4]);
        return HandType::TwoPair;
    };

    let is_one_pair = {
        card_check[0] == card_check[1]
        || card_check[1] == card_check[2]
        || card_check[2] == card_check[3]
        || card_check[3] == card_check[4]
    };

    if is_one_pair {
        println!("[{:?}{:?}{:?}{:?}{:?}] - OnePair", card_check[0], card_check[1], card_check[2], card_check[3], card_check[4]);
        return HandType::OnePair;
    };

    println!("[{:?}{:?}{:?}{:?}{:?}] - HighCard", card_check[0], card_check[1], card_check[2], card_check[3], card_check[4]);
    HandType::HighCard

}

impl Hand {
    fn parse(input: String) -> Hand {
        let components: Vec<&str> = input.split(' ').collect();

        let cards: Vec<Card> = components[0].chars().map(Card::parse).collect();
        let bid = str::parse(components[1]).expect("oops no bid!!!");

        Hand { ty: compute_hand_type(&cards), cards, bid }
    }
}


pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let lines: Vec<String> = in_file.lines().map(|x| x.expect("boo! bad string")).collect();
    let mut hands: Vec<Hand> = lines.into_iter().map(Hand::parse).collect();
    hands.sort();
    hands.reverse();

    //println!("hands: {:#?}", hands);

    let total_winnings: usize = hands.iter().enumerate().map(|(rank, hand)| (rank + 1) * hand.bid).sum();

    println!("total winnings: {}", total_winnings);

    Ok(())
}