#![allow(dead_code)]
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut hands = get_hands("input.txt");
    hands.sort();

    // 248909434
    println!("{:?}", hands);
    println!("{:?}", get_winnings(hands));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    J,
    N(u8),
    T,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    bid: i32,
    cards: Vec<Card>,
    rank: Rank,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank == other.rank {
            for i in 0..self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    return Some(self.cards[i].cmp(&other.cards[i]));
                }
            }
        }
        Some(self.rank.cmp(&other.rank))
    }
}

impl Hand {
    fn from(input: String) -> Self {
        let input: Vec<&str> = input.split_whitespace().collect();
        let cards: Vec<Card> = input[0]
            .chars()
            .map(|card| match card {
                'T' => Card::T,
                'J' => Card::J,
                'Q' => Card::Q,
                'K' => Card::K,
                'A' => Card::A,
                _ => Card::N(card.to_string().parse::<u8>().unwrap()),
            })
            .collect();

        let mut hand = Self {
            cards,
            bid: input[1].parse::<i32>().unwrap(),
            rank: Rank::HighCard,
        };

        hand.get_type();
        hand
    }

    fn get_type(&mut self) {
        let mut frequency_map: HashMap<&Card, i32> = HashMap::new();
        for card in &self.cards {
            frequency_map
                .entry(card)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        // number of J wildcards
        let j_count = frequency_map.remove(&Card::J).unwrap_or(0);
        // add the jokers to the card that appears the most
        let mut max: (&Card, &i32) = (&Card::J, &0);
        for key in frequency_map.keys() {
            let count = frequency_map.get(key).unwrap();
            if count > &max.1 {
                max = (key, count);
            }
        }
        frequency_map.insert(max.0, max.1 + j_count);

        // five of a kind
        if frequency_map.values().count() == 1 || frequency_map.values().count() == 0 {
            self.rank = Rank::FiveOfAKind;
            return;
        }
        // four of a kind
        if frequency_map.values().any(|&v| v == 4) {
            self.rank = Rank::FourOfAKind;
            return;
        }
        // full house
        if frequency_map.values().any(|&v| v == 3) && frequency_map.values().any(|&v| v == 2) {
            self.rank = Rank::FullHouse;
            return;
        }
        // three of a kind
        if frequency_map.values().any(|&v| v == 3) {
            self.rank = Rank::ThreeOfAKind;
            return;
        }
        // two pair
        if frequency_map.values().filter(|&v| v == &2).count() == 2 {
            self.rank = Rank::TwoPair;
            return;
        }
        // one pair
        if frequency_map.values().any(|&v| v == 2) || j_count == 1 {
            self.rank = Rank::OnePair;
            return;
        }
        // high card
        self.rank = Rank::HighCard;
    }
}

fn get_hands(file: &str) -> Vec<Hand> {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .map(|line| Hand::from(line))
        .collect()
}

fn get_winnings(hands: Vec<Hand>) -> i32 {
    hands
        .iter()
        .enumerate()
        .fold(0, |wins, (rank, hand)| wins + hand.bid * (rank + 1) as i32)
}
