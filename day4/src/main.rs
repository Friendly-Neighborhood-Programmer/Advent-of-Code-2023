#![allow(dead_code)]
use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let cards = get_input("input.txt");
    // Part 1
    // let points = get_points(&cards);
    // println!("total points: {:?}", points);

    // Part 2
    let total = total_cards(&cards);
    println!("total card count: {:?}", total);
}

#[derive(Debug)]
struct Card {
    id: i32,
    winning: HashSet<i32>,
    numbers: Vec<i32>,
    points: i32,
}

impl Card {
    fn from(line: String) -> Self {
        let id = line.find(':').unwrap();
        let split = line.find('|').unwrap();

        let winning: HashSet<i32> = line[id + 1..split]
            .split(' ')
            .filter(|l| l != &"")
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect();

        let numbers: Vec<i32> = line[split + 1..]
            .split(' ')
            .filter(|l| l != &"")
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect();

        let mut card = Self {
            id: line[5..id].replace(" ", "").parse::<i32>().unwrap(),
            winning,
            numbers,
            points: 0,
        };

        card.calc_points();
        card
    }

    fn calc_points(&mut self) {
        for num in &self.numbers {
            if self.winning.contains(num) {
                self.points += 1;
            }
        }
    }
}

fn get_input(file: &str) -> Vec<Card> {
    BufReader::new(File::open(file).expect("input file not found"))
        .lines()
        .map(Result::unwrap)
        .map(|line| Card::from(line))
        .collect()
}

// Part 1
fn get_points(cards: &Vec<Card>) -> i32 {
    cards.into_iter().fold(0, |sum, card| sum + card.points)
}

// Part 2
fn total_cards(cards: &Vec<Card>) -> usize {
    let mut card_count = 0;
    let mut queue: VecDeque<usize> = VecDeque::new();

    for i in 1..=cards.len() {
        queue.push_back(i)
    }

    while let Some(index) = queue.pop_front() {
        for i in 1..=cards[index - 1].points {
            queue.push_back(index + i as usize);
        }
        card_count += 1;
    }

    card_count
}
