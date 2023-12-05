use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

#[derive(Clone)]
struct Card {
    card_id: i64,
    winning_numbers: HashMap<i64, ()>,
    my_numbers: Vec<i64>
}

impl Card {
    fn new(line: &str) -> Card {
        let chunks = line.split(':').collect::<Vec<&str>>();
    
        let card_splits = chunks[0].split(' ').collect::<Vec<&str>>();
        let card_id = str::parse::<i64>(card_splits[card_splits.len()-1]).expect("failed to extract card id");
        
        let number_splits = chunks[1].split('|').collect::<Vec<&str>>();

        let winning_numbers = number_splits[0].strip_prefix(' ').unwrap_or(number_splits[0]).split_whitespace().map(|x| str::parse::<i64>(x).expect("failed to parse")).collect::<Vec<i64>>();
        let my_numbers= number_splits[1].strip_prefix(' ').unwrap_or(number_splits[1]).split_whitespace().map(|x| str::parse::<i64>(x).expect("failed to parse")).collect::<Vec<i64>>();

        let mut winning_number_set = HashMap::new();

        for number in winning_numbers {
            winning_number_set.insert(number, ());
        }
    
        Card { card_id, winning_numbers: winning_number_set, my_numbers }
    }

    fn score(&self) -> i64 {
        let mut score: i64 = 0;
        let mut num_matchs = 0;

        for number in &self.my_numbers {
            if self.winning_numbers.contains_key(number) {
                if score == 0 {
                    score = 1;
                } else {
                    num_matchs += 1;
                }
            }
        }

        for _ in 0..num_matchs {
            score *= 2;
        }

        score
    }

    fn num_matchs(&self) -> i64 {
        let mut num_matchs = 0;
        for number in &self.my_numbers {
            if self.winning_numbers.contains_key(number) {
                num_matchs += 1;
            }
        }

        num_matchs
    }
    
}


pub fn question_one(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let mut cards = Vec::new();
    for line in in_file.lines() {
        let validated_line = line?;

        cards.push(Card::new(&validated_line));
    }

    let score: i64 = cards.iter().map(|x| x.score()).sum();

    println!("total: {}", score);

    Ok(())
}

pub fn question_two(in_file: BufReader<File>) -> Result<(), std::io::Error> {
    let mut card_map = HashMap::new();
    let mut cards = Vec::new();
    for line in in_file.lines() {
        let validated_line = line?;

        let cc = Card::new(&validated_line);
        card_map.insert(cc.card_id, (cc.clone(), 1));
        cards.push(cc.clone());
    }

    for card in cards.iter() {
        let num_matches = card.num_matchs();

        let number_of_cards = card_map.get(&card.card_id).expect("failed to find known card count").1;

        for mm in 1..=num_matches {
            if let Some(cc) = card_map.get_mut(&(card.card_id+mm)) {
                cc.1 += number_of_cards;
            };
        }
    }

    let total_cards: i32 = card_map.values().map(|x| x.1).sum();

    println!("number of cards: {}", total_cards);

    Ok(())
}